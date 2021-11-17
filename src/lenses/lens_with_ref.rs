use crate::lenses::{
    lens,
    lens::FuncLens,
    lens_from_arc,
    to::{to_ref, to_ref_from_arc, ToRefInner},
    Lens, LensOver, LensView,
};
use std::sync::Arc;

#[derive(Clone, Copy)]
/// Lens that works on both a T and &T
pub struct LensWithRef<A, B, T>(A, B, std::marker::PhantomData<T>);

impl<'a, A, B, T> LensView<T> for LensWithRef<Lens<A>, B, T>
where
    T: Clone,
    A: LensView<T>,
{
    type Field = A::Field;

    fn view(&self, thing: T) -> Self::Field {
        A::view(&self.0 .0, thing)
    }
}
impl<'a, A, B, T> LensOver<T> for LensWithRef<Lens<A>, B, T>
where
    T: Clone,
    A: LensOver<T>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        A::over(&self.0 .0, thing, f)
    }
}

impl<'a, A, B, T> LensView<&'a T> for LensWithRef<A, Lens<B>, T>
where
    T: Clone,
    A: LensView<T>,
    B: LensView<&'a T>,
{
    type Field = B::Field;

    fn view(&self, thing: &'a T) -> Self::Field {
        B::view(&self.1 .0, thing)
    }
}

type Getter<T, U> = dyn Fn(T) -> U;
type Setter<T, U> = dyn Fn(T, U) -> T;

/// Makes a lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
pub fn lens_with_ref_from_arc<T, U>(
    getter: Arc<Getter<T, U>>,
    setter: Arc<Setter<T, U>>,
    getter_ref: Arc<dyn Fn(&T) -> U>,
) -> Lens<LensWithRef<Lens<FuncLens<T, U>>, Lens<ToRefInner<T, U>>, T>>
where
    T: Clone,
{
    Lens(LensWithRef(
        lens_from_arc(getter, setter),
        to_ref_from_arc(getter_ref),
        Default::default(),
    ))
}
/// Makes a lens that implements `LensView<T>` and `LensOver<T>` with the provided functions
pub fn lens_with_ref<T, U>(
    getter: impl Fn(T) -> U + 'static,
    setter: impl Fn(T, U) -> T + 'static,
    getter_ref: impl Fn(&T) -> U + 'static,
) -> Lens<LensWithRef<Lens<FuncLens<T, U>>, Lens<ToRefInner<T, U>>, T>>
where
    T: Clone,
{
    Lens(LensWithRef(
        lens(getter, setter),
        to_ref(getter_ref),
        Default::default(),
    ))
}
