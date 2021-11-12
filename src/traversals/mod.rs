pub mod both;
pub use both::both;

pub mod each;
pub use each::each;

pub mod head;
pub use head::_head;
pub mod tail;
pub use tail::_tail;

use crate::{
    lenses::{Lens, LensOver, LensView},
    prisms::{Prism, PrismPreview},
};

/// Wrapper type
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Traversal<T>(pub(crate) T);

pub trait TraversalTraverse<T> {
    type Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field>;
}
pub trait TraversalOver<T>: TraversalTraverse<T> {
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field;

    fn set(&self, thing: T, v: Self::Field) -> T
    where
        Self::Field: Clone,
    {
        Self::over(self, thing, move |_| v.clone())
    }
}

impl<L, T> TraversalTraverse<T> for Traversal<L>
where
    L: TraversalTraverse<T>,
{
    type Field = L::Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field> {
        L::traverse(&self.0, thing)
    }
}
impl<L, T> TraversalOver<T> for Traversal<L>
where
    L: TraversalOver<T>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        L::over(&self.0, thing, f)
    }
}

// all lenses are traversals, so we can freely transform them into a traversal
impl<L> Lens<L> {
    /// Returns this lens as a traversal
    pub const fn to_traversal(self) -> Traversal<Lens<L>> {
        Traversal(self)
    }
}
// we can go back to a lens from a "traversal-ed" lens
impl<L> Traversal<Lens<L>> {
    /// Returns the wrapped lens
    pub fn to_lens(self) -> Lens<L> {
        self.0
    }
}
impl<L, T> TraversalTraverse<T> for Lens<L>
where
    L: LensView<T>,
{
    type Field = L::Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field> {
        vec![L::view(&self.0, thing)]
    }
}
impl<L, T> TraversalOver<T> for Lens<L>
where
    L: LensView<T> + LensOver<T>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        L::over(&self.0, thing, f)
    }
}

// all prisms are traversals, so we can freely transform them into a traversal
impl<L> Prism<L> {
    /// Returns this lens as a traversal
    pub const fn to_traversal(self) -> Traversal<Prism<L>> {
        Traversal(self)
    }
}
// we can go back to a lens from a "traversal-ed" lens
impl<L> Traversal<Prism<L>> {
    /// Returns the wrapped lens
    pub fn to_prism(self) -> Prism<L> {
        self.0
    }
}
impl<L, T> TraversalTraverse<T> for Prism<L>
where
    L: PrismPreview<T>,
{
    type Field = L::Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field> {
        L::preview(&self.0, thing).into_iter().collect()
    }
}
impl<L, T> TraversalOver<T> for Prism<L>
where
    T: Clone,
    L: PrismPreview<T>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        L::over(&self.0, thing, f)
    }
}

pub fn traverse<T, L: TraversalTraverse<T>>(lens: L, thing: T) -> Vec<L::Field> {
    L::traverse(&lens, thing)
}
pub fn set<T, L: TraversalOver<T>>(lens: L, thing: T, v: L::Field) -> T
where
    <L as TraversalTraverse<T>>::Field: Clone,
{
    L::set(&lens, thing, v)
}
pub fn over<T, L: TraversalOver<T>>(lens: L, thing: T, f: impl FnMut(L::Field) -> L::Field) -> T {
    L::over(&lens, thing, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse_each_works_on_arrays() {
        let array = [1, 2, 3, 4];

        let res = each(array);
        assert_eq!(res, vec![1, 2, 3, 4,]);

        let res = each(array, |v| v + 1);
        assert_eq!(res, [2, 3, 4, 5]);
    }

    #[test]
    fn traverse_head_works_on_arrays() {
        let array = [1, 2, 3, 4];

        let res = _head(array);
        assert_eq!(res, vec![1,]);

        let res = _head(array, |v| v + 1);
        assert_eq!(res, [2, 2, 3, 4,]);
    }

    #[test]
    fn traverse_tail_works_on_arrays() {
        let array = [1, 2, 3, 4];

        let res = _tail(array);
        assert_eq!(res, vec![2, 3, 4]);

        let res = _tail(array, |v| v + 1);
        assert_eq!(res, [1, 3, 4, 5]);
    }
}
