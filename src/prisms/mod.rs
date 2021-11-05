use crate::{Optics, OpticsTrait};

/// For lenses that allow viewing
pub trait PrismPreview<T>: OpticsTrait {
    type Field;

    fn preview(thing: T) -> Option<Self::Field>;
}
pub trait PrismReview<T>: PrismPreview<T> {
    fn review(thing: Self::Field) -> T;
}

impl<P, T> PrismPreview<T> for Optics<P>
where
    P: PrismPreview<T>,
{
    type Field = P::Field;

    fn preview(thing: T) -> Option<Self::Field> {
        P::preview(thing)
    }
}

impl<P, T> PrismReview<T> for Optics<P>
where
    P: PrismReview<T>,
{
    fn review(thing: Self::Field) -> T {
        P::review(thing)
    }
}

pub fn preview<T, P: PrismPreview<T>>(_prism: P, thing: T) -> Option<P::Field> {
    P::preview(thing)
}
pub fn review<T, P: PrismReview<T>>(_prism: P, thing: P::Field) -> T {
    P::review(thing)
}

mod result;
pub use result::{_Err, _Ok};

mod option;
pub use option::{_None, _Some};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_result() {
        let a: Result<i32, i32> = Ok(3);
        assert_eq!(preview(_Ok, a), Some(3));

        let a: Result<i32, i32> = Err(3);
        assert_eq!(preview(_Ok, a), None);

        let a: Result<i32, i32> = Ok(3);
        assert_eq!(preview(_Err, a), None);

        let a: Result<i32, i32> = Err(3);
        assert_eq!(preview(_Err, a), Some(3));
    }

    #[test]
    fn preview_option() {
        let a = Some(3);
        assert_eq!(preview(_Some, a), Some(3));

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
    fn view_combination() {}
}
