mod identity;
pub use identity::id;

mod first;
pub use first::_0;
mod second;
pub use second::_1;

mod to;
pub use to::{to, to_from_boxed};
mod lens;
pub use lens::{lens, lens_from_boxed};

use crate::traversals::Traversal;

/// Wrapper type
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Lens<L>(pub(crate) L);

// all lenses are traversals, so we can freely transform them into a traversal
impl<L> Lens<L> {
    /// Returns this lens as a traversal
    pub fn to_traversal(self) -> Traversal<Lens<L>> {
        Traversal(self)
    }
}
// we can go back to a lens from a "traversal-ed" lens
impl<L> Traversal<Lens<L>> {
    /// Returns the wrapped lens
    pub fn to_lens(self) -> Lens<L> {
        self.0
    }
}

/// For lenses that allow viewing
pub trait LensView<T> {
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

impl<L, T> LensView<T> for Lens<L>
where
    L: LensView<T>,
{
    type Field = L::Field;

    fn view(&self, thing: T) -> Self::Field {
        L::view(&self.0, thing)
    }
}
impl<L, T> LensOver<T> for Lens<L>
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

    #[derive(Debug, PartialEq, Clone)]
    struct Hello {
        hey: u8,
    }

    #[test]
    fn can_use_to() {
        // making a getter
        let l = to(|hello: Hello| hello.hey);

        let hello = Hello { hey: 8 };
        assert_eq!(l(hello), 8);
    }

    #[test]
    fn can_make_lens_out_of_funcs() {
        // making a lens
        let l = lens(
            |hello: Hello| hello.hey,
            |mut hello: Hello, v: u8| {
                hello.hey = v;
                hello
            },
        );

        let hello = Hello { hey: 8 };
        assert_eq!(l(hello), 8);

        let hello = Hello { hey: 8 };
        assert_eq!(l(hello, |v| v + 1), Hello { hey: 9 });
    }

    #[test]
    fn can_make_lens_out_of_to() {
        // we first use to, and then use that to make a full lens

        let l = to(|hello: Hello| hello.hey);
        let l = l.make_lens(|mut hello: Hello, v: u8| {
            hello.hey = v;
            hello
        });

        let hello = Hello { hey: 8 };
        assert_eq!(l(hello), 8);

        let hello = Hello { hey: 8 };
        assert_eq!(l(hello, |v| v + 1), Hello { hey: 9 });
    }

    #[test]
    fn convoluted_example() {
        #[derive(Debug, PartialEq, Clone)]
        struct Hello2 {
            hey: (u8, (u8, i32)),
        }

        // make a lens for Hello
        let l = lens(
            |hello: Hello2| hello.hey,
            |mut hello: Hello2, v| {
                hello.hey = v;
                hello
            },
        );

        let thing = Hello2 { hey: (1, (2, -3)) };
        let thing = (thing, "hello");

        let lens_that_targets_the_i32 = _0 + l + _1 + _1;

        assert_eq!(lens_that_targets_the_i32(thing), -3);
    }
}
