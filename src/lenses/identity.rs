use crate::{LensOver, LensTrait, LensView};

#[allow(non_camel_case_types)]
pub struct id;

impl LensTrait for id {}
impl<T> LensView<T> for id {
    type Field = T;

    fn view(thing: T) -> Self::Field {
        thing
    }
}
impl<T> LensOver<T> for id {
    fn over<F>(thing: T, f: F) -> T
    where
        F: FnOnce(Self::Field) -> Self::Field,
    {
        f(thing)
    }
}
