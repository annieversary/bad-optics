use crate::{
    lenses::{LensOver, LensView},
    Optics, OpticsTrait,
};

type Getter<T, U> = dyn Fn(T) -> U;
type Setter<T, U> = dyn Fn(T, U) -> T;

pub struct LensInner<T, U>(pub(crate) Box<Getter<T, U>>, pub(crate) Box<Setter<T, U>>);
impl<T, U> OpticsTrait for LensInner<T, U> {}

impl<T, U> LensView<T> for LensInner<T, U> {
    type Field = U;

    fn view(&self, thing: T) -> Self::Field {
        (self.0)(thing)
    }
}
impl<T: Clone, U> LensOver<T> for LensInner<T, U> {
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        let v = f((self.0)(thing.clone()));
        (self.1)(thing, v)
    }

    fn set(&self, thing: T, v: Self::Field) -> T {
        (self.1)(thing, v)
    }
}

/// Makes a lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
pub fn lens_from_boxed<T, U>(
    getter: Box<Getter<T, U>>,
    setter: Box<Setter<T, U>>,
) -> Optics<LensInner<T, U>> {
    Optics(LensInner(getter, setter))
}
/// Makes a lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
pub fn lens<T, U>(
    getter: impl Fn(T) -> U + 'static,
    setter: impl Fn(T, U) -> T + 'static,
) -> Optics<LensInner<T, U>> {
    Optics(LensInner(Box::new(getter), Box::new(setter)))
}
