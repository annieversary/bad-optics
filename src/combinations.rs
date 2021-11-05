use crate::{Lens, LensOver, LensTrait, LensView};

#[derive(Clone, Copy)]
pub struct Combination<A, B>(A, B);
impl<A, B> LensTrait for Combination<A, B> {}

impl<A, B> std::ops::Add<Lens<B>> for Lens<A>
where
    A: LensTrait,
    B: LensTrait,
{
    type Output = Lens<Combination<Lens<A>, Lens<B>>>;

    fn add(self, rhs: Lens<B>) -> Self::Output {
        Lens(Combination(self, rhs))
    }
}

impl<A, B, T> LensView<T> for Combination<A, B>
where
    A: LensView<T>,
    B: LensView<A::Field>,
{
    type Field = B::Field;

    fn view(thing: T) -> Self::Field {
        B::view(A::view(thing))
    }
}

impl<A, B, T> LensOver<T> for Combination<A, B>
where
    A: LensOver<T>,
    B: LensOver<A::Field>,
{
    fn over<F>(thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        A::over(thing, |b| B::over(b, f))
    }
}
