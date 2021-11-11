use crate::{lenses::LensView, Optics, OpticsTrait};

use super::lens::LensInner;

pub struct ToInner<T, U>(Box<dyn Fn(T) -> U>);
impl<T, U> OpticsTrait for ToInner<T, U> {}

impl<T, U> LensView<T> for ToInner<T, U> {
    type Field = U;

    fn view(&self, thing: T) -> Self::Field {
        (self.0)(thing)
    }
}

/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to_from_boxed<T, U>(f: Box<dyn Fn(T) -> U>) -> Optics<ToInner<T, U>> {
    Optics(ToInner(f))
}
/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to<T, U>(f: impl Fn(T) -> U + 'static) -> Optics<ToInner<T, U>> {
    Optics(ToInner(Box::new(f)))
}

impl<T, U> Optics<ToInner<T, U>> {
    /// Makes a full lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
    pub fn make_lens(self, setter: impl Fn(T, U) -> T + 'static) -> Optics<LensInner<T, U>> {
        Optics(LensInner((self.0).0, Box::new(setter)))
    }
}
