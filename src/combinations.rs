use crate::{
    lenses::{LensOver, LensView},
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
