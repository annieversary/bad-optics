use crate::{
    lenses::{LensOver, LensView},
    prisms::PrismPreview,
    Optics, OpticsTrait,
};

#[derive(Clone, Copy)]
pub struct Combination<A, B>(A, B);
impl<A, B> OpticsTrait for Combination<A, B> {}

impl<A, B> std::ops::Add<Optics<B>> for Optics<A>
where
    A: OpticsTrait,
    B: OpticsTrait,
{
    type Output = Optics<Combination<Optics<A>, Optics<B>>>;

    fn add(self, rhs: Optics<B>) -> Self::Output {
        Optics(Combination(self, rhs))
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

impl<A, B, T> PrismPreview<T> for Combination<A, B>
where
    A: LensView<T>,
    B: PrismPreview<A::Field>,
{
    type Field = B::Field;

    fn preview(thing: T) -> Option<Self::Field> {
        B::preview(A::view(thing))
    }
}
