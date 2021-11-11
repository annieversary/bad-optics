use crate::{
    lenses::{LensOver, LensView},
    OpticsTrait,
};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct id;

impl OpticsTrait for id {}
impl<T> LensView<T> for id {
    type Field = T;

    fn view(&self, thing: T) -> Self::Field {
        thing
    }
}
impl<T> LensOver<T> for id {
    fn over<F>(&self, thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        f(thing)
    }
}
