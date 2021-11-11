use crate::lenses::{Lens, LensOver, LensView};

#[derive(Clone, Copy)]
pub struct IdInner;
#[allow(non_upper_case_globals)]
pub const id: Lens<IdInner> = Lens(IdInner);

impl<T> LensView<T> for IdInner {
    type Field = T;

    fn view(&self, thing: T) -> Self::Field {
        thing
    }
}
impl<T> LensOver<T> for IdInner {
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        f(thing)
    }
}
