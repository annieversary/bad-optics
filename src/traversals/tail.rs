use crate::traversals::{Traversal, TraversalOver, TraversalTraverse};

#[derive(Clone, Copy)]
pub struct TailInner;
#[allow(non_upper_case_globals)]
pub const _tail: Traversal<TailInner> = Traversal(TailInner);

macro_rules! make_tuples {
    (( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl<T> TraversalTraverse<( $($t,)* )> for TailInner {
            type Field = T;

            fn traverse(&self, ( _, $($v,)* ): ($($t,)*)) -> Vec<Self::Field> {
                vec![$($v,)*]
            }
        }
        impl<T> TraversalOver<( $($t,)* )> for TailInner {
            fn over<F>(
                &self,
                (head, $($v,)*): ($($t,)*),
                mut fun: F
            ) -> ( $($t,)* )
            where
              F: FnMut(Self::Field) -> Self::Field
            {
                (head, $( fun($v) ,)*)
            }
        }

        impl<'a, T> TraversalTraverse<&'a ( $($t,)* )> for TailInner {
            type Field = &'a T;

            fn traverse(&self, (_, $($v,)* ): &'a ($($t,)*)) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
        impl<'a, T> TraversalTraverse<&'a mut ( $($t,)* )> for TailInner {
            type Field = &'a mut T;

            fn traverse(&self, (_, $($v,)* ): &'a mut ($($t,)*)) -> Vec<Self::Field> {
                vec![ $($v,)* ]
            }
        }
    };
}

make_tuples!((_u), (T, T));
make_tuples!((_u, _v), (T, T, T));
make_tuples!((_u, _v, _w), (T, T, T, T));
make_tuples!((_u, _v, _w, _x), (T, T, T, T, T));
make_tuples!((_u, _v, _w, _x, _y), (T, T, T, T, T, T));
make_tuples!((_u, _v, _w, _x, _y, _z), (T, T, T, T, T, T, T));

macro_rules! make_arrays {
    ($n:expr, [$( $v:ident ),*]) => {
        impl<T> TraversalTraverse<[T; $n]> for TailInner {
            type Field = T;

            fn traverse(&self, [ _,  $($v,)* ]: [T; $n]) -> Vec<Self::Field> {
                vec![$($v,)*]
            }
        }
        impl<T> TraversalOver<[T; $n]> for TailInner {
            fn over<F>(
              &self,
              [ head, $($v,)* ]: [T; $n],
              mut fun: F
            ) -> [T; $n]
            where
              F: FnMut(Self::Field) -> Self::Field
            {
              [head, $(fun($v),)*]
            }
        }

        impl<'a, T> TraversalTraverse<&'a [T; $n]> for TailInner {
            type Field = &'a T;

            fn traverse(&self, [_, $($v,)* ]: &'a [T; $n]) -> Vec<Self::Field> {
                vec![$($v,)*]
            }
        }
        impl<'a, T> TraversalTraverse<&'a mut [T; $n]> for TailInner {
            type Field = &'a mut T;

            fn traverse(&self, [_, $($v,)* ]: &'a mut [T; $n]) -> Vec<Self::Field> {
                vec![$($v,)*]
            }
        }
    };
}

make_arrays!(2, [_a]);
make_arrays!(3, [_a, _b]);
make_arrays!(4, [_a, _b, _c]);
make_arrays!(5, [_a, _b, _c, _d]);
make_arrays!(6, [_a, _b, _c, _d, _e]);
make_arrays!(7, [_a, _b, _c, _d, _e, _g]);
