use crate::lenses::{Lens, LensOver, LensView};

#[derive(Clone, Copy)]
pub struct _3Inner;
pub const _3: Lens<_3Inner> = Lens(_3Inner);

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl< $($t,)* > LensView<( $($t,)* )> for _3Inner {
            type Field = T;

            fn view(&self, ( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl< $($t,)* > LensOver<( $($t,)* )> for _3Inner {
            fn over<F>(
                &self,
                mut tup: ($($t,)*),
                f: F
            ) -> ( $($t,)* )
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
                tup.3 = f(tup.3);
                tup
            }
        }

        impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _3Inner {
            type Field = &'a T;

            fn view(&self, ( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _3Inner {
            type Field = &'a mut T;

            fn view(&self, ( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make_tuples!(t, (_u, _v, _w, t), (U, V, W, T));
make_tuples!(t, (_u, _v, _w, t, _x), (U, V, W, T, X));
make_tuples!(t, (_u, _v, _w, t, _x, _y), (U, V, W, T, X, Y));
make_tuples!(t, (_u, _v, _w, t, _x, _y, _z), (U, V, W, T, X, Y, Z));
make_tuples!(t, (_u, _v, _w, t, _x, _y, _z, _a), (U, V, W, T, X, Y, Z, A));
make_tuples!(
    t,
    (_u, _v, _w, t, _x, _y, _z, _a, _b),
    (U, V, W, T, X, Y, Z, A, B)
);
make_tuples!(
    t,
    (_u, _v, _w, t, _x, _y, _z, _a, _b, _c),
    (U, V, W, T, X, Y, Z, A, B, C)
);
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> LensView<[T; $n]> for _3Inner {
            type Field = T;

            fn view(&self, [ $($v,)* ]: [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<T> LensOver<[T; $n]> for _3Inner {
            fn over<F>(
              &self,
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

      impl<'a, T> LensView<&'a [T; $n]> for _3Inner {
            type Field = &'a T;

            fn view(&self, [ $($v,)* ]: &'a [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<'a, T> LensView<&'a mut [T; $n]> for _3Inner {
            type Field = &'a mut T;

            fn view(&self, [ $($v,)* ]: &'a mut [T; $n]) -> Self::Field {
                $f
            }
        }
    };
}

make_arrays!(t, 4, [_a, _b, _c, t]);
make_arrays!(t, 5, [_a, _b, _c, t, _d]);
make_arrays!(t, 6, [_a, _b, _c, t, _d, _e]);
make_arrays!(t, 7, [_a, _b, _c, t, _d, _e, _g]);
make_arrays!(t, 8, [_a, _b, _c, t, _d, _e, _g, _h]);
make_arrays!(t, 9, [_a, _b, _c, t, _d, _e, _g, _h, _i]);
