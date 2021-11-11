use super::*;

#[derive(Clone, Copy)]
pub struct OkInner;
#[allow(non_upper_case_globals)]
pub const _Ok: Prism<OkInner> = Prism(OkInner);
impl<T, E> PrismPreview<Result<T, E>> for OkInner {
    type Field = T;

    fn preview(&self, thing: Result<T, E>) -> Option<Self::Field> {
        thing.ok()
    }
    fn review(&self, thing: Self::Field) -> Result<T, E> {
        Ok(thing)
    }
}

#[derive(Clone, Copy)]
pub struct ErrInner;
#[allow(non_upper_case_globals)]
pub const _Err: Prism<ErrInner> = Prism(ErrInner);

impl<T, E> PrismPreview<Result<T, E>> for ErrInner {
    type Field = E;

    fn preview(&self, thing: Result<T, E>) -> Option<Self::Field> {
        thing.err()
    }
    fn review(&self, thing: Self::Field) -> Result<T, E> {
        Err(thing)
    }
}
