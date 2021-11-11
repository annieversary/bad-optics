mod both;
pub use both::both;

mod each;
pub use each::each;

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
    use super::*;

    #[test]
    fn traverse_each_works_on_arrays() {
        let array = [1, 2, 3, 4];

        let res = each(array, |v| v + 1);
        assert_eq!(res, [2, 3, 4, 5]);
    }
}
