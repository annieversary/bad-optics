mod identity;
pub use identity::id;

mod first;
pub use first::_0;
mod second;
pub use second::_1;

mod to;

use crate::{Optics, OpticsTrait};

/// For lenses that allow viewing
pub trait LensView<T>: OpticsTrait {
    type Field;

    fn view(&self, thing: T) -> Self::Field;
}

/// For lenses that allow setting
pub trait LensOver<T>: LensView<T> {
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field;

    fn set(&self, thing: T, v: Self::Field) -> T {
        Self::over(self, thing, |_| v)
    }
}

impl<L, T> LensView<T> for Optics<L>
where
    L: LensView<T>,
{
    type Field = L::Field;

    fn view(&self, thing: T) -> Self::Field {
        L::view(&self.0, thing)
    }
}
impl<L, T> LensOver<T> for Optics<L>
where
    L: LensOver<T>,
{
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        L::over(&self.0, thing, f)
    }
}

pub fn view<T, L: LensView<T>>(lens: L, thing: T) -> L::Field {
    L::view(&lens, thing)
}
pub fn set<T, L: LensOver<T>>(lens: L, thing: T, v: L::Field) -> T {
    L::set(&lens, thing, v)
}
pub fn over<T, L: LensOver<T>>(lens: L, thing: T, f: impl FnOnce(L::Field) -> L::Field) -> T {
    L::over(&lens, thing, f)
}

#[cfg(test)]
mod tests {
    use super::*;

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
