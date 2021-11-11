use super::*;

#[derive(Clone, Copy)]
pub struct SomeInner;
#[allow(non_upper_case_globals)]
pub const _Some: Prism<SomeInner> = Prism(SomeInner);

impl<T> PrismPreview<Option<T>> for SomeInner {
    type Field = T;

    fn preview(&self, thing: Option<T>) -> Option<Self::Field> {
        thing
    }

    fn review(&self, thing: Self::Field) -> Option<T> {
        Some(thing)
    }
}

#[derive(Clone, Copy)]
pub struct NoneInner;
#[allow(non_upper_case_globals)]
pub const _None: Prism<NoneInner> = Prism(NoneInner);
impl<T> PrismPreview<Option<T>> for NoneInner {
    type Field = ();

    fn preview(&self, _thing: Option<T>) -> Option<Self::Field> {
        Some(())
    }

    fn review(&self, _thing: Self::Field) -> Option<T> {
        None
    }
}
