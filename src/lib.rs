#![feature(unboxed_closures, fn_traits, const_trait_impl)]
#![allow(clippy::type_complexity)]

pub mod combinations;
mod fns;
pub mod lenses;
pub mod prisms;
pub mod traversals;

pub mod prelude {
    pub use bad_optics_derive::Optics;

    pub use crate::combinations::*;

    pub use crate::lenses::*;
    pub use crate::prisms::*;
    pub use crate::traversals::*;

    pub use crate::has_lens::*;
}

pub mod has_lens {
    use crate::prelude::{lens::FuncLens, lens_with_ref::LensWithRef, to::ToRefInner};

    use super::prelude::*;

    pub trait HasLens {
        type Lenses;
    }

    pub trait HasLensOf<T>: Sized {
        fn get() -> Vec<Lens<LensWithRef<Lens<FuncLens<Self, T>>, Lens<ToRefInner<Self, T>>, Self>>>;
    }

    #[macro_export]
    macro_rules! lens {
        ($name:ident :: $func:ident) => {
            <$name as HasLens>::Lenses::$func()
        };
    }
    #[macro_export]
    macro_rules! lenses {
        ($name:ident :: $ty:ident) => {
            <$name as HasLensOf<$ty>>::get()
        };
    }
}
