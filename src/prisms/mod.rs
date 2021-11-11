mod result;
pub use result::{_Err, _Ok};

mod option;
pub use option::{_None, _Some};

/// Wrapper type
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Prism<P>(pub(crate) P);

pub trait PrismPreview<T> {
    type Field;

    fn preview(&self, thing: T) -> Option<Self::Field>;
    fn review(&self, thing: Self::Field) -> T;
    // TODO id like for this to not need clone
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
        T: Clone,
    {
        if let Some(a) = Self::preview(&self, thing.clone()) {
            Self::review(&self, f(a))
        } else {
            thing
        }
    }

    fn set(&self, thing: T, v: Self::Field) -> T
    where
        T: Clone,
        Self::Field: Clone,
    {
        Self::over(self, thing, move |_| v.clone())
    }
}

impl<P, T> PrismPreview<T> for Prism<P>
where
    P: PrismPreview<T>,
{
    type Field = P::Field;

    fn preview(&self, thing: T) -> Option<Self::Field> {
        P::preview(&self.0, thing)
    }

    fn review(&self, thing: Self::Field) -> T {
        P::review(&self.0, thing)
    }
}

pub fn preview<T, P: PrismPreview<T>>(prism: P, thing: T) -> Option<P::Field> {
    P::preview(&prism, thing)
}
pub fn review<T, P: PrismPreview<T>>(prism: P, thing: P::Field) -> T {
    P::review(&prism, thing)
}
pub fn over<T: Clone, P: PrismPreview<T>>(
    prism: P,
    thing: T,
    f: impl FnOnce(P::Field) -> P::Field,
) -> T {
    P::over(&prism, thing, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_result() {
        let a: Result<i32, i32> = Ok(3);
        assert_eq!(_Ok(a), Some(3));

        let a: Result<i32, i32> = Err(3);
        assert_eq!(preview(_Ok, a), None);

        let a: Result<i32, i32> = Ok(3);
        assert_eq!(_Err(a), None);

        let a: Result<i32, i32> = Err(3);
        assert_eq!(preview(_Err, a), Some(3));
    }

    #[test]
    fn preview_option() {
        let a = Some(3);
        assert_eq!(_Some(a), Some(3));

        let a = Some(3);
        assert_eq!(preview(_None, a), Some(()));

        let a: Option<i32> = None;
        assert_eq!(preview(_Some, a), None);

        let a: Option<i32> = None;
        assert_eq!(preview(_None, a), Some(()));
    }

    #[test]
    fn review_result() {
        assert_eq!(review(_Ok, 3), Ok::<i32, i32>(3));
        assert_eq!(review(_Err, 3), Err::<i32, i32>(3));
    }

    #[test]
    fn review_option() {
        assert_eq!(review(_Some, 3), Some(3));
        assert_eq!(review(_None, ()), None::<()>);
    }

    #[test]
    fn over_option() {
        assert_eq!(over(_Some, Some(3), |v| v + 1), Some(4));
        assert_eq!(_Some(Some(3), |v| v + 1), Some(4));
        assert_eq!(over(_None, None, |_v: ()| ()), None::<()>);
    }
}
