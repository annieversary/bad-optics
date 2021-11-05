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
pub mod prisms;
