#![feature(unboxed_closures, fn_traits)]

/// Base trait
pub trait LensTrait {}

/// For lenses that allow viewing
pub trait LensView<T>: LensTrait {
    type Field;

    fn view(thing: T) -> Self::Field;
}

/// For lenses that allow setting
pub trait LensOver<T>: LensView<T> {
    fn over<F>(thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field;

    fn set(thing: T, v: Self::Field) -> T {
        Self::over(thing, |_| v)
    }
}

/// Wrapper type
#[derive(Clone, Copy)]
pub struct Lens<T: LensTrait>(T);

impl<L: LensTrait> LensTrait for Lens<L> {}
impl<L, T> LensView<T> for Lens<L>
where
    L: LensView<T>,
{
    type Field = L::Field;

    fn view(thing: T) -> Self::Field {
        L::view(thing)
    }
}
impl<L, T> LensOver<T> for Lens<L>
where
    L: LensOver<T>,
{
    fn over<F>(thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        L::over(thing, f)
    }
}

pub fn view<T, L: LensView<T>>(_lens: L, thing: T) -> L::Field {
    L::view(thing)
}
pub fn set<T, L: LensOver<T>>(_lens: L, thing: T, v: L::Field) -> T {
    L::set(thing, v)
}
pub fn over<T, L: LensOver<T>>(_lens: L, thing: T, f: impl FnOnce(L::Field) -> L::Field) -> T {
    L::over(thing, f)
}

// TODO add fn impls

// TODO add third_from_tuple, etc

// TODO array traversals

// TODO make over work with changing types

mod combinations;
mod fns;
pub mod lenses;

#[cfg(test)]
mod tests {
    use super::{
        lenses::{_0, _1},
        *,
    };

    #[test]
    fn view_first_from_tuple() {
        let a = (1, 2);
        assert_eq!(view(_0, a), 1);

        let a = (1, 2);
        assert_eq!(view(_0, &a), &1);

        let mut a = (1, 2);
        assert_eq!(view(_0, &mut a), &mut 1);

        let a = (1, 2, 3);
        assert_eq!(view(_0, a), 1);

        let a = (1, 2, 3);
        assert_eq!(view(_0, &a), &1);
    }

    #[test]
    fn view_first_from_tuple_mut_works() {
        let mut a = (1, 2);
        *view(_0, &mut a) += 1;

        assert_eq!(a, (2, 2));
    }

    #[test]
    fn set_first_from_tuple() {
        let a = (1, 2);
        let a = set(_0, a, 3);
        assert_eq!(a, (3, 2));
    }

    #[test]
    fn over_first_from_tuple() {
        let a = (1, 2);
        let a = over(_0, a, |v| v + 1);
        assert_eq!(a, (2, 2));
    }

    #[test]
    fn over_first_from_array() {
        let a = [1, 2, 3, 4];

        let a = over(_0, a, |v| v + 1);
        assert_eq!(a, [2, 2, 3, 4]);
    }

    #[test]
    fn second() {
        let a = (1, 2);
        let a = over(_1, a, |v| v + 1);
        assert_eq!(a, (1, 3));

        let a = [1, 2, 3, 4];
        let a = over(_1, a, |v| v + 1);
        assert_eq!(a, [1, 3, 3, 4]);
    }

    #[test]
    fn view_combination() {
        let a = ((1, 2), 3);

        let lens = _0 + _1;
        let a = view(lens, a);
        assert_eq!(a, 2);
    }

    #[test]
    fn over_combination() {
        let a = ((1, 2), 3);

        let lens = _0 + _1;
        let a = over(lens, a, |v| v + 1);
        assert_eq!(a, ((1, 3), 3));
    }

    #[test]
    fn call_as_funcs() {
        let a = (1, 2);
        assert_eq!(_0(a), 1);

        let a = (1, 2);
        assert_eq!(_0(a, |v| v + 1), (2, 2));

        let a = ((1, 2), 3);
        let lens = _0 + _1;

        let res = lens(a);
        assert_eq!(res, 2);
        let res = lens(a, |v| v + 1);
        assert_eq!(res, ((1, 3), 3));
    }
}
