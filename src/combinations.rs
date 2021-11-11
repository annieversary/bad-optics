use crate::lenses::{Lens, LensOver, LensView};

#[derive(Clone, Copy)]
pub struct Combination<A, B>(A, B);

impl<A, B> std::ops::Add<Lens<B>> for Lens<A> {
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
