use crate::traversals::{Traversal, TraversalOver, TraversalTraverse};

#[derive(Clone, Copy)]
pub struct BothInner;
#[allow(non_upper_case_globals)]
pub const both: Traversal<BothInner> = Traversal(BothInner);

impl<T> TraversalTraverse<(T, T)> for BothInner {
    type Field = T;

    fn traverse(&self, (a, b): (T, T)) -> Vec<Self::Field> {
        vec![a, b]
    }
}

impl<T> TraversalOver<(T, T)> for BothInner {
    fn over<F>(&self, (a, b): (T, T), mut f: F) -> (T, T)
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        (f(a), f(b))
    }
}
