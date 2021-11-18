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
}
