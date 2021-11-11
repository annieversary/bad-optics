use crate::{
    traversals::{TraversalOver, TraversalTraverse},
    Optics, OpticsTrait,
};

#[derive(Clone, Copy)]
pub struct EachInner;
#[allow(non_upper_case_globals)]
pub const each: Optics<EachInner> = Optics(EachInner);
impl OpticsTrait for EachInner {}

impl<T> TraversalTraverse<Vec<T>> for EachInner {
    type Field = T;

    fn traverse(&self, thing: Vec<T>) -> Vec<Self::Field> {
        thing
    }
}

impl<T> TraversalOver<Vec<T>> for EachInner {
    fn over<F>(&self, thing: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Field) -> Self::Field,
    {
        thing.into_iter().map(f).collect()
    }
}

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl<T> TraversalTraverse<( $($t,)* )> for EachInner {
            type Field = T;

            fn traverse(&self, ( $($v,)* ): ($($t,)*)) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
        impl<T> TraversalOver<( $($t,)* )> for EachInner {
            fn over<F>(
                &self,
                ($($v,)*): ($($t,)*),
                mut f: F
            ) -> ( $($t,)* )
            where
              F: FnMut(Self::Field) -> Self::Field
            {

              ( $( f($v), )* )
            }
        }

        impl<'a, T> TraversalTraverse<&'a ( $($t,)* )> for EachInner {
            type Field = &'a T;

            fn traverse(&self, ( $($v,)* ): &'a ($($t,)*)) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
        impl<'a, T> TraversalTraverse<&'a mut ( $($t,)* )> for EachInner {
            type Field = &'a mut T;

            fn traverse(&self, ( $($v,)* ): &'a mut ($($t,)*)) -> Vec<Self::Field> {
              vec![ $($v,)* ]
            }
        }
    };
}

make_tuples!(t, (t), (T));
make_tuples!(t, (t, u), (T, T));
make_tuples!(t, (t, u, v), (T, T, T));
make_tuples!(t, (t, u, v, w), (T, T, T, T));
make_tuples!(t, (t, u, v, w, x), (T, T, T, T, T));
make_tuples!(t, (t, u, v, w, x, y), (T, T, T, T, T, T));
make_tuples!(t, (t, u, v, w, x, y, z), (T, T, T, T, T, T, T));
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> TraversalTraverse<[T; $n]> for EachInner {
            type Field = T;

            fn traverse(&self, [ $($v,)* ]: [T; $n]) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
      impl<T> TraversalOver<[T; $n]> for EachInner {
            fn over<F>(
              &self,
              [ $($v,)* ]: [T; $n],
              mut fun: F
            ) -> [T; $n]
            where
              F: FnMut(Self::Field) -> Self::Field
            {
              [$(fun($v),)*]
            }
        }

      impl<'a, T> TraversalTraverse<&'a [T; $n]> for EachInner {
            type Field = &'a T;

            fn traverse(&self, [ $($v,)* ]: &'a [T; $n]) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
      impl<'a, T> TraversalTraverse<&'a mut [T; $n]> for EachInner {
            type Field = &'a mut T;

            fn traverse(&self, [ $($v,)* ]: &'a mut [T; $n]) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
    };
}

make_arrays!(t, 1, [t]);
make_arrays!(t, 2, [t, _a]);
make_arrays!(t, 3, [t, _a, _b]);
make_arrays!(t, 4, [t, _a, _b, _c]);
make_arrays!(t, 5, [t, _a, _b, _c, _d]);
make_arrays!(t, 6, [t, _a, _b, _c, _d, _e]);
make_arrays!(t, 7, [t, _a, _b, _c, _d, _e, _g]);
