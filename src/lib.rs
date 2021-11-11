#![feature(unboxed_closures, fn_traits)]

/// Base trait
pub trait OpticsTrait {}

/// Wrapper type
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Optics<T: OpticsTrait>(pub(crate) T);
impl<L: OpticsTrait> OpticsTrait for Optics<L> {}

mod combinations;
mod fns;
pub mod lenses;
pub mod prisms;
pub mod traversals;
