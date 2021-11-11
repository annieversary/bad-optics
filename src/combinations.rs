use crate::{
    lenses::{Lens, LensOver, LensView},
    prisms::Prism,
    traversals::{Traversal, TraversalOver, TraversalTraverse},
};

#[derive(Clone, Copy)]
pub struct Combination<A, B>(A, B);

// additions

// lens + lens
impl<A, B> std::ops::Add<Lens<B>> for Lens<A> {
    type Output = Lens<Combination<Lens<A>, Lens<B>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Lens(Combination(self, rhs))
    }
}
// traversal + traversal
impl<A, B> std::ops::Add<Traversal<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self, rhs))
    }
}
// traversal + lens
impl<A, B> std::ops::Add<Lens<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<Lens<B>>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Traversal(Combination(self, rhs.to_traversal()))
    }
}
// lens + traversal
impl<A, B> std::ops::Add<Traversal<B>> for Lens<A> {
    type Output = Traversal<Combination<Traversal<Lens<A>>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs))
    }
}
// traversal + prism
impl<A, B> std::ops::Add<Prism<B>> for Traversal<A> {
    type Output = Traversal<Combination<Traversal<A>, Traversal<Prism<B>>>>;

    fn add(self, rhs: Prism<B>) -> Self::Output {
        Traversal(Combination(self, rhs.to_traversal()))
    }
}
// prism + traversal
impl<A, B> std::ops::Add<Traversal<B>> for Prism<A> {
    type Output = Traversal<Combination<Traversal<Prism<A>>, Traversal<B>>>;

    fn add(self, rhs: Traversal<B>) -> Self::Output {
        Traversal(Combination(self.to_traversal(), rhs))
    }
}

// trait impls for Combination

impl<A, B, T> LensView<T> for Combination<A, B>
where
    A: LensView<T>,
    B: LensView<A::Field>,
{
    type Field = B::Field;

    fn view(&self, thing: T) -> Self::Field {
        B::view(&self.1, A::view(&self.0, thing))
    }
}

impl<A, B, T> LensOver<T> for Combination<A, B>
where
    A: LensOver<T>,
    B: LensOver<A::Field>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        A::over(&self.0, thing, |b| B::over(&self.1, b, f))
    }
}

impl<A, B, T> TraversalTraverse<T> for Combination<A, B>
where
    A: TraversalTraverse<T>,
    B: TraversalTraverse<A::Field>,
{
    type Field = B::Field;

    fn traverse(&self, thing: T) -> Vec<Self::Field> {
        let a = A::traverse(&self.0, thing);
        a.into_iter()
            .map(|v| B::traverse(&self.1, v))
            .flatten()
            .collect()
    }
}

impl<A, B, T> TraversalOver<T> for Combination<A, B>
where
    A: TraversalOver<T>,
    B: TraversalOver<A::Field>,
{
    fn over<F>(&self, thing: T, mut f: F) -> T
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        A::over(&self.0, thing, |b| B::over(&self.1, b, &mut f))
    }
}
