use crate::{
    lenses::{LensOver, LensView},
    Optics,
};

impl<L, A> std::ops::FnOnce<(A,)> for Optics<L>
where
    L: LensView<A>,
{
    type Output = L::Field;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        L::view(args.0)
    }
}
impl<L, A> std::ops::FnMut<(A,)> for Optics<L>
where
    L: LensView<A>,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        L::view(args.0)
    }
}
impl<L, A> std::ops::Fn<(A,)> for Optics<L>
where
    L: LensView<A>,
{
    extern "rust-call" fn call(&self, args: (A,)) -> Self::Output {
        L::view(args.0)
    }
}

impl<L, A, F> std::ops::FnOnce<(A, F)> for Optics<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    type Output = A;

    extern "rust-call" fn call_once(self, args: (A, F)) -> Self::Output {
        L::over(args.0, args.1)
    }
}
impl<L, A, F> std::ops::FnMut<(A, F)> for Optics<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    extern "rust-call" fn call_mut(&mut self, args: (A, F)) -> Self::Output {
        L::over(args.0, args.1)
    }
}
impl<L, A, F> std::ops::Fn<(A, F)> for Optics<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    extern "rust-call" fn call(&self, args: (A, F)) -> Self::Output {
        L::over(args.0, args.1)
    }
}
