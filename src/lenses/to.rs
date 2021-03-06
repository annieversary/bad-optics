use std::sync::Arc;

use crate::lenses::{Lens, LensView};

use super::lens::FuncLens;

#[derive(Clone)]
pub struct ToInner<T, U>(Arc<dyn Fn(T) -> U>);

impl<T, U> LensView<T> for ToInner<T, U> {
    type Field = U;

    fn view(&self, thing: T) -> Self::Field {
        (self.0)(thing)
    }
}

/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to_from_arc<T, U>(f: Arc<dyn Fn(T) -> U>) -> Lens<ToInner<T, U>> {
    Lens(ToInner(f))
}
/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to<T, U>(f: impl Fn(T) -> U + 'static) -> Lens<ToInner<T, U>> {
    Lens(ToInner(Arc::new(f)))
}

impl<T, U> Lens<ToInner<T, U>> {
    /// Makes a full lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
    pub fn make_lens(self, setter: impl Fn(T, U) -> T + 'static) -> Lens<FuncLens<T, U>> {
        Lens(FuncLens((self.0).0, Arc::new(setter)))
    }
}

#[derive(Clone)]
pub struct ToRefInner<T, U>(Arc<dyn Fn(&T) -> U>);

impl<T, U> LensView<&T> for ToRefInner<T, U> {
    type Field = U;

    fn view(&self, thing: &T) -> Self::Field {
        (self.0)(thing)
    }
}

/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to_ref_from_arc<T, U>(f: Arc<dyn Fn(&T) -> U>) -> Lens<ToRefInner<T, U>> {
    Lens(ToRefInner(f))
}
/// Makes a lens that implements `LensView<T>` with the provided function
pub fn to_ref<T, U>(f: impl Fn(&T) -> U + 'static) -> Lens<ToRefInner<T, U>> {
    Lens(ToRefInner(Arc::new(f)))
}
