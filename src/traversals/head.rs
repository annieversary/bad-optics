use crate::traversals::{Traversal, TraversalOver, TraversalTraverse};

#[derive(Clone, Copy)]
pub struct HeadInner;
#[allow(non_upper_case_globals)]
pub const _head: Traversal<HeadInner> = Traversal(HeadInner);

macro_rules! make_tuples {
    (( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl< $($t,)* > TraversalTraverse<( $($t,)* )> for HeadInner {
            type Field = T;

            fn traverse(&self, ( head, $($v,)* ): ($($t,)*)) -> Vec<Self::Field> {
                vec![head]
            }
        }
        impl< $($t,)* > TraversalOver<( $($t,)* )> for HeadInner {
            fn over<F>(
                &self,
                (head, $($v,)*): ($($t,)*),
                mut f: F
            ) -> ( $($t,)* )
            where
              F: FnMut(Self::Field) -> Self::Field
            {

                ( f(head), $($v,)* )
            }
        }

        impl<'a, $($t,)* > TraversalTraverse<&'a ( $($t,)* )> for HeadInner {
            type Field = &'a T;

            fn traverse(&self, (head, $($v,)* ): &'a ($($t,)*)) -> Vec<Self::Field> {
                vec![ head ]
            }
        }
        impl<'a, $($t,)* > TraversalTraverse<&'a mut ( $($t,)* )> for HeadInner {
            type Field = &'a mut T;

            fn traverse(&self, (head, $($v,)* ): &'a mut ($($t,)*)) -> Vec<Self::Field> {
              vec![ head ]
            }
        }
    };
}

make_tuples!((), (T));
make_tuples!((_u), (T, U));
make_tuples!((_u, _v), (T, U, V));
make_tuples!((_u, _v, _w), (T, U, V, W));
make_tuples!((_u, _v, _w, _x), (T, U, V, W, X));
make_tuples!((_u, _v, _w, _x, _y), (T, U, V, W, X, Y));
make_tuples!((_u, _v, _w, _x, _y, _z), (T, U, V, W, X, Y, Z));

macro_rules! make_arrays {
    ($n:expr, [$( $v:ident ),*]) => {
      impl<T> TraversalTraverse<[T; $n]> for HeadInner {
            type Field = T;

            fn traverse(&self, [ head,  $($v,)* ]: [T; $n]) -> Vec<Self::Field> {
                vec![head]
            }
        }
      impl<T> TraversalOver<[T; $n]> for HeadInner {
            fn over<F>(
              &self,
              [ head, $($v,)* ]: [T; $n],
              mut fun: F
            ) -> [T; $n]
            where
              F: FnMut(Self::Field) -> Self::Field
            {
              [fun(head), $(($v),)*]
            }
        }

      impl<'a, T> TraversalTraverse<&'a [T; $n]> for HeadInner {
            type Field = &'a T;

            fn traverse(&self, [head, $($v,)* ]: &'a [T; $n]) -> Vec<Self::Field> {
                vec![head]
            }
        }
      impl<'a, T> TraversalTraverse<&'a mut [T; $n]> for HeadInner {
            type Field = &'a mut T;

            fn traverse(&self, [head, $($v,)* ]: &'a mut [T; $n]) -> Vec<Self::Field> {
                vec![head]
            }
        }
    };
}

make_arrays!(1, []);
make_arrays!(2, [_a]);
make_arrays!(3, [_a, _b]);
make_arrays!(4, [_a, _b, _c]);
make_arrays!(5, [_a, _b, _c, _d]);
make_arrays!(6, [_a, _b, _c, _d, _e]);
make_arrays!(7, [_a, _b, _c, _d, _e, _g]);
