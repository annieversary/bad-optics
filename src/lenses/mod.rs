mod identity;
pub use identity::id;

mod first;
pub use first::_0;
mod second;
pub use second::_1;

use crate::{Optics, OpticsTrait};

/// For lenses that allow viewing
pub trait LensView<T>: OpticsTrait {
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

impl<L, T> LensView<T> for Optics<L>
where
    L: LensView<T>,
{
    type Field = L::Field;

    fn view(thing: T) -> Self::Field {
        L::view(thing)
    }
}
impl<L, T> LensOver<T> for Optics<L>
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
