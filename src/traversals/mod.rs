mod both;
pub use both::both;

mod each;
pub use each::each;

use crate::lenses::{Lens, LensOver, LensView};

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
    pub fn to_traversal(self) -> Traversal<Lens<L>> {
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
    use crate::lenses::_0;

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
    fn can_combine_traversals() {
        let array = [vec![1, 2], vec![3, 4]];

        // combine two traversals
        let res = (each + each)(array, |v| v + 1);
        assert_eq!(res, [vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn can_combine_traversal_with_lens() {
        let array = [(1, 2), (3, 4), (5, 6)];

        // combine a traversal with a lens
        let t = each + _0;

        // traverse
        let res = t(array);
        assert_eq!(res, vec![1, 3, 5]);

        // over
        let res = t(array, |v| v + 1);
        assert_eq!(res, [(2, 2), (4, 4), (6, 6)]);
    }

    #[test]
    fn can_combine_lens_with_traversal() {
        let array = [(1, 2), (3, 4), (5, 6)];

        // combine a traversal with a lens
        let t = _0 + each;

        // traverse
        let res = t(array);
        assert_eq!(res, vec![1, 2]);

        // over
        let res = t(array, |v| v + 1);
        assert_eq!(res, [(2, 3), (3, 4), (5, 6)]);
    }
}
