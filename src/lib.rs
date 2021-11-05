#![feature(unboxed_closures, fn_traits)]

// TODO add third_from_tuple, etc

// TODO array traversals

// TODO something for structs

// TODO something for Ok, Some, etc

// TODO make over work with changing types

/// Base trait
pub trait OpticsTrait {}

/// Wrapper type
#[derive(Clone, Copy)]
pub struct Optics<T: OpticsTrait>(pub(crate) T);
impl<L: OpticsTrait> OpticsTrait for Optics<L> {}

mod combinations;
mod fns;
pub mod lenses;

#[cfg(test)]
mod tests {
    use super::lenses::*;

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

        let res = _0(a);
        assert_eq!(res, (1, 2));

        let lens = _0 + _1;

        let res = lens(a);
        assert_eq!(res, 2);
        let res = lens(a, |v| v + 1);
        assert_eq!(res, ((1, 3), 3));
    }
}
