#![feature(unboxed_closures, fn_traits)]

pub mod combinations;
mod fns;
pub mod lenses;
pub mod prisms;
pub mod traversals;

pub mod prelude {
    pub use crate::combinations::*;

    pub use crate::lenses::*;
    pub use crate::prisms::*;
    pub use crate::traversals::*;
}
