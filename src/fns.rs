use crate::{
    lenses::{Lens, LensOver, LensView},
    traversals::{Traversal, TraversalOver, TraversalTraverse},
};

// lens view
impl<L, A> std::ops::FnOnce<(A,)> for Lens<L>
where
    L: LensView<A>,
{
    type Output = L::Field;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        L::view(&self.0, args.0)
    }
}
impl<L, A> std::ops::FnMut<(A,)> for Lens<L>
where
    L: LensView<A>,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        L::view(&self.0, args.0)
    }
}
impl<L, A> std::ops::Fn<(A,)> for Lens<L>
where
    L: LensView<A>,
{
    extern "rust-call" fn call(&self, args: (A,)) -> Self::Output {
        L::view(&self.0, args.0)
    }
}

// lens over
impl<L, A, F> std::ops::FnOnce<(A, F)> for Lens<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    type Output = A;

    extern "rust-call" fn call_once(self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}
impl<L, A, F> std::ops::FnMut<(A, F)> for Lens<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    extern "rust-call" fn call_mut(&mut self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}
impl<L, A, F> std::ops::Fn<(A, F)> for Lens<L>
where
    L: LensOver<A>,
    F: FnOnce(L::Field) -> L::Field,
{
    extern "rust-call" fn call(&self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}

// traversal traverse
impl<L, A> std::ops::FnOnce<(A,)> for Traversal<L>
where
    L: TraversalTraverse<A>,
{
    type Output = Vec<L::Field>;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        L::traverse(&self.0, args.0)
    }
}
impl<L, A> std::ops::FnMut<(A,)> for Traversal<L>
where
    L: TraversalTraverse<A>,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        L::traverse(&self.0, args.0)
    }
}
impl<L, A> std::ops::Fn<(A,)> for Traversal<L>
where
    L: TraversalTraverse<A>,
{
    extern "rust-call" fn call(&self, args: (A,)) -> Self::Output {
        L::traverse(&self.0, args.0)
    }
}

// traversal over
impl<L, A, F> std::ops::FnOnce<(A, F)> for Traversal<L>
where
    L: TraversalOver<A>,
    F: FnMut(L::Field) -> L::Field,
{
    type Output = A;

    extern "rust-call" fn call_once(self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}
impl<L, A, F> std::ops::FnMut<(A, F)> for Traversal<L>
where
    L: TraversalOver<A>,
    F: FnMut(L::Field) -> L::Field,
{
    extern "rust-call" fn call_mut(&mut self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}
impl<L, A, F> std::ops::Fn<(A, F)> for Traversal<L>
where
    L: TraversalOver<A>,
    F: FnMut(L::Field) -> L::Field,
{
    extern "rust-call" fn call(&self, args: (A, F)) -> Self::Output {
        L::over(&self.0, args.0, args.1)
    }
}
