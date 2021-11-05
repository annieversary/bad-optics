use super::*;

#[derive(Clone, Copy)]
pub struct SomeInner;
#[allow(non_upper_case_globals)]
pub const _Some: Optics<SomeInner> = Optics(SomeInner);
impl OpticsTrait for SomeInner {}
impl<T> PrismPreview<Option<T>> for SomeInner {
    type Field = T;

    fn preview(thing: Option<T>) -> Option<Self::Field> {
        thing
    }
}

impl<T> PrismReview<Option<T>> for SomeInner {
    fn review(thing: Self::Field) -> Option<T> {
        Some(thing)
    }
}

#[derive(Clone, Copy)]
pub struct NoneInner;
#[allow(non_upper_case_globals)]
pub const _None: Optics<NoneInner> = Optics(NoneInner);
impl OpticsTrait for NoneInner {}
impl<T> PrismPreview<Option<T>> for NoneInner {
    type Field = ();

    fn preview(_thing: Option<T>) -> Option<Self::Field> {
        Some(())
    }
}

impl<T> PrismReview<Option<T>> for NoneInner {
    fn review(_thing: Self::Field) -> Option<T> {
        None
    }
}
