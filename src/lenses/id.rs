use crate::{LensOver, LensView};

#[allow(non_camel_case_types)]
pub struct id;

impl<T> LensView<T> for T {
    type Field = T;

    fn view(thing: T) -> Self::Field {
        thing
    }
}
impl<T> LensOver<T> for T {
    fn over(thing: T, f: &dyn Fn(Self::Field) -> Self::Field) -> T {
        f(thing)
    }
}
