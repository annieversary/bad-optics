use crate::{
    lenses::{Lens, LensOver, LensView},
    prisms::{Prism, PrismPreview},
    traversals::{Traversal, TraversalOver, TraversalTraverse},
};
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Combination<A, B>(A, B);

// additions

// lens + lens
impl<A, B> const Add<Lens<B>> for Lens<A> {
    type Output = Lens<Combination<Lens<A>, Lens<B>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Lens(Combination(self, rhs))
    }
}
// prism + prism
impl<A, B> const Add<Prism<B>> for Prism<A> {
    type Output = Prism<Combination<Prism<A>, Prism<B>>>;

    fn add(self, rhs: Prism<B>) -> Self::Output {
        Prism(Combination(self, rhs))
    }
}
// traversal + traversal
impl<A, B> const Add<Traversal<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self, rhs))
    }
}
// traversal + lens
impl<A, B> const Add<Lens<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<Lens<B>>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Traversal(Combination(self, rhs.to_traversal()))
    }
}
// lens + traversal
impl<A, B> const Add<Traversal<B>> for Lens<A> {
    type Output = Traversal<Combination<Traversal<Lens<A>>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs))
    }
}
// traversal + prism
impl<A, B> const Add<Prism<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<Prism<B>>>>;

    fn add(self, rhs: Prism<B>) -> Self::Output {
        Traversal(Combination(self, rhs.to_traversal()))
    }
}
// prism + traversal
impl<A, B> const Add<Traversal<B>> for Prism<A> {
    type Output = Traversal<Combination<Traversal<Prism<A>>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs))
    }
}
// lens + prism
impl<A, B> const Add<Prism<B>> for Lens<A> {
    type Output = Traversal<Combination<Traversal<Lens<A>>, Traversal<Prism<B>>>>;

    fn add(self, rhs: Prism<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs.to_traversal()))
    }
}
// prism + traversal
impl<A, B> const Add<Lens<B>> for Prism<A> {
    type Output = Traversal<Combination<Traversal<Prism<A>>, Traversal<Lens<B>>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs.to_traversal()))
    }
}

// trait impls for Combination

// lens + lens
impl<A, B, T> LensView<T> for Combination<Lens<A>, Lens<B>>
where
    A: LensView<T>,
    B: LensView<A::Field>,
{
    type Field = B::Field;

    fn view(&self, thing: T) -> Self::Field {
        B::view(&self.1 .0, A::view(&self.0 .0, thing))
    }
}
impl<A, B, T> LensOver<T> for Combination<Lens<A>, Lens<B>>
where
    A: LensOver<T>,
    B: LensOver<A::Field>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        A::over(&self.0 .0, thing, |b| B::over(&self.1 .0, b, f))
    }
}

// prism + prism
impl<A, B, T> PrismPreview<T> for Combination<Prism<A>, Prism<B>>
where
    A: PrismPreview<T>,
    B: PrismPreview<A::Field>,
{
    type Field = B::Field;

    fn preview(&self, thing: T) -> Option<Self::Field> {
        A::preview(&self.0 .0, thing).and_then(|a| B::preview(&self.1 .0, a))
    }

    fn review(&self, thing: Self::Field) -> T {
        A::review(&self.0 .0, B::review(&self.1 .0, thing))
    }
}

// traversal + traversal
// lens + traversal
// traversal + lens
// prism + traversal
// traversal + prism
// prism + lens
// lens + prism
impl<A, B, T> TraversalTraverse<T> for Combination<Traversal<A>, Traversal<B>>
where
    A: TraversalTraverse<T>,
    B: TraversalTraverse<A::Field>,
{
    type Field = B::Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field> {
        let a = A::traverse(&self.0 .0, thing);
        a.into_iter()
            .map(|v| B::traverse(&self.1 .0, v))
            .flatten()
            .collect()
    }
}
impl<A, B, T> TraversalOver<T> for Combination<Traversal<A>, Traversal<B>>
where
    A: TraversalOver<T>,
    B: TraversalOver<A::Field>,
{
    fn over<F>(&self, thing: T, mut f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        A::over(&self.0 .0, thing, |b| B::over(&self.1 .0, b, &mut f))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lenses::{_0, _1},
        prisms::_Some,
        traversals::each,
    };

    #[test]
    fn can_view_lens_combination() {
        let a = ((1, 2), 3);

        let lens = _0 + _1;
        let a = lens(a);
        assert_eq!(a, 2);
    }

    #[test]
    fn can_over_lens_combination() {
        let a = ((1, 2), 3);

        let lens = _0 + _1;
        let a = lens(a, |v| v + 1);
        assert_eq!(a, ((1, 3), 3));
    }

    #[test]
    fn can_combine_prisms() {
        let thing = Some(Some(3));

        // combine two traversals
        let res = (_Some + _Some)(thing, |v| v + 1);
        assert_eq!(res, Some(Some(4)));
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

    #[test]
    fn can_combine_prism_with_traversal() {
        let array = [Some(1), None, Some(3), None, Some(5)];

        // combine a traversal with a lens
        let t = each + _Some;

        // traverse
        let res = t(array);
        assert_eq!(res, vec![1, 3, 5]);

        // over
        let res = t(array, |v| v + 1);
        assert_eq!(res, [Some(2), None, Some(4), None, Some(6)]);
    }

    #[test]
    fn can_combine_traversal_with_prism() {
        let array = Some([1, 2, 3]);

        // combine a traversal with a lens
        let t = _Some + each;

        // traverse
        let res = t(array);
        assert_eq!(res, vec![1, 2, 3]);

        // over
        let res = t(array, |v| v + 1);
        assert_eq!(res, Some([2, 3, 4]));

        let array: Option<[i32; 3]> = None;
        // traverse
        let res = t(array);
        assert_eq!(res, vec![]);

        // over
        let res = t(array, |v| v + 1);
        assert_eq!(res, None);
    }

    #[test]
    fn can_combine_prism_with_lens() {
        let thing = Some((1, 2));

        // combine a traversal with a lens
        let t = _Some + _0;

        // NOTE: combination of a prism and a lens is a traversal
        //
        // > The optic kind resulting from a composition is the least upper bound (join)
        // > of the optic kinds being composed, if it exists.
        // > The Join type family computes the least upper bound given two optic kind tags.
        // > For example the Join of a Lens and a Prism is an AffineTraversal.
        //
        // from: https://hackage.haskell.org/package/optics-0.4/docs/Optics.html

        // traversal
        let res = t(thing);
        assert_eq!(res, vec![1]);

        // over
        let res = t(thing, |v| v + 1);
        assert_eq!(res, Some((2, 2)));
    }

    #[test]
    fn can_combine_lens_with_prism() {
        let thing = (Some(1), 2);

        // combine a traversal with a lens
        let t = _0 + _Some;

        // NOTE: combination of a lens and a prism is a traversal
        // see can_combine_prism_with_lens for more info

        // traversal
        let res = t(thing);
        assert_eq!(res, vec![1]);

        // over
        let res = t(thing, |v| v + 1);
        assert_eq!(res, (Some(2), 2));
    }

    #[test]
    fn can_combine_as_const() {
        use crate::lenses::first::_0Inner;
        use crate::lenses::Lens;
        const LENS: Lens<super::Combination<Lens<_0Inner>, Lens<_0Inner>>> = _0 + _0;

        let thing: ((i32, i32), i32) = ((1, 2), 3);

        let r: i32 = LENS(thing);

        assert_eq!(r, 1);
    }
}
