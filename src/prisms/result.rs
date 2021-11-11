use super::*;

#[derive(Clone, Copy)]
pub struct OkInner;
#[allow(non_upper_case_globals)]
pub const _Ok: Prism<OkInner> = Prism(OkInner);
impl<T, E> PrismPreview<Result<T, E>> for OkInner {
    type Field = T;

    fn preview(thing: Result<T, E>) -> Option<Self::Field> {
        thing.ok()
    }
}

impl<T, E> PrismReview<Result<T, E>> for OkInner {
    fn review(thing: Self::Field) -> Result<T, E> {
        Ok(thing)
    }
}

#[derive(Clone, Copy)]
pub struct ErrInner;
#[allow(non_upper_case_globals)]
pub const _Err: Prism<ErrInner> = Prism(ErrInner);

impl<T, E> PrismPreview<Result<T, E>> for ErrInner {
    type Field = E;

    fn preview(thing: Result<T, E>) -> Option<Self::Field> {
        thing.err()
    }
}

impl<T, E> PrismReview<Result<T, E>> for ErrInner {
    fn review(thing: Self::Field) -> Result<T, E> {
        Err(thing)
    }
}
