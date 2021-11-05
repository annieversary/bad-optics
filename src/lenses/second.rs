use crate::{Lens, LensOver, LensTrait, LensView};

#[derive(Clone, Copy)]
pub struct _1Inner;
pub const _1: Lens<_1Inner> = Lens(_1Inner);
impl LensTrait for _1Inner {}

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
      impl< $($t,)* > LensView<( $($t,)* )> for _1Inner {
            type Field = T;

            fn view(( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }
      impl< $($t,)* > LensOver<( $($t,)* )> for _1Inner {
            fn over<F>(
                mut tup: ($($t,)*),
                f: F
            ) -> ( $($t,)* )
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
                tup.1 = f(tup.1);
                tup
            }
        }

      impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _1Inner {
            type Field = &'a T;

            fn view(( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
      impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _1Inner {
            type Field = &'a mut T;

            fn view(( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make_tuples!(t, (_u, t), (U, T));
make_tuples!(t, (_u, t, _v), (U, T, V));
make_tuples!(t, (_u, t, _v, _w), (U, T, V, W));
make_tuples!(t, (_u, t, _v, _w, _x), (U, T, V, W, X));
make_tuples!(t, (_u, t, _v, _w, _x, _y), (U, T, V, W, X, Y));
make_tuples!(t, (_u, t, _v, _w, _x, _y, _z), (U, T, V, W, X, Y, Z));
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> LensView<[T; $n]> for _1Inner {
            type Field = T;

            fn view([ $($v,)* ]: [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<T> LensOver<[T; $n]> for _1Inner {
            fn over<F>(
              tup: [T; $n],
              fun: F
            ) -> [T; $n]
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
              let [$($v,)*] = tup;
              let $f = fun( $f );
              [$($v,)*]
            }
        }

      impl<'a, T> LensView<&'a [T; $n]> for _1Inner {
            type Field = &'a T;

            fn view([ $($v,)* ]: &'a [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<'a, T> LensView<&'a mut [T; $n]> for _1Inner {
            type Field = &'a mut T;

            fn view([ $($v,)* ]: &'a mut [T; $n]) -> Self::Field {
                $f
            }
        }
    };
}

make_arrays!(t, 2, [_a, t]);
make_arrays!(t, 3, [_a, t, _b]);
make_arrays!(t, 4, [_a, t, _b, _c]);
make_arrays!(t, 5, [_a, t, _b, _c, _d]);
make_arrays!(t, 6, [_a, t, _b, _c, _d, _e]);
make_arrays!(t, 7, [_a, t, _b, _c, _d, _e, _g]);
