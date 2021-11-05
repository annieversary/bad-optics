pub trait LensView<T> {
    type Field;

    fn view(thing: T) -> Self::Field;
}

pub trait LensOver<T>: LensView<T> {
    fn over(thing: T, f: &dyn Fn(Self::Field) -> Self::Field) -> T;

    fn set(thing: T, v: Self::Field) -> T {
        Self::over(thing, &|_| v)
    }
}

pub fn view<T, L: LensView<T>>(_lens: L, thing: T) -> L::Field {
    L::view(thing)
}
pub fn set<T, L: LensOver<T>>(_lens: L, thing: T, v: L::Field) -> T {
    L::set(thing, v)
}
pub fn over<T, L: LensOver<T>>(_lens: L, thing: T, f: &dyn Fn(L::Field) -> L::Field) -> T {
    L::over(thing, f)
}

// TODO add Fn implementation for lenses
//      with one param it should be view, with two it should be over
// TODO add std::ops::Add to combine lenses

// TODO add second_from_tuple, etc

// TODO array traversals

// TODO make over work with changing types

pub mod lenses;

#[cfg(test)]
mod tests {
    use super::{lenses::_0, *};

    #[test]
    fn view_first_from_tuple() {
        let a = (1, 2);
        assert_eq!(view(_0, a), 1);

        // you can call it both ways
        let a = (1, 2);
        assert_eq!(_0::view(a), 1);

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

        let a = (1, 2);
        let a = _0::set(a, 3);
        assert_eq!(a, (3, 2));
    }

    #[test]
    fn over_first_from_tuple() {
        let a = (1, 2);
        let a = over(_0, a, &|v| v + 1);
        assert_eq!(a, (2, 2));

        let a = (1, 2);
        let a = _0::over(a, &|v| v + 1);
        assert_eq!(a, (2, 2));
    }
}
