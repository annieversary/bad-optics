use crate::lenses::{Lens, LensOver, LensView};

#[derive(Clone, Copy)]
pub struct _4Inner;
pub const _4: Lens<_4Inner> = Lens(_4Inner);

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl< $($t,)* > LensView<( $($t,)* )> for _4Inner {
            type Field = T;

            fn view(&self, ( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl< $($t,)* > LensOver<( $($t,)* )> for _4Inner {
            fn over<F>(
                &self,
                mut tup: ($($t,)*),
                f: F
            ) -> ( $($t,)* )
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
                tup.4 = f(tup.4);
                tup
            }
        }

        impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _4Inner {
            type Field = &'a T;

            fn view(&self, ( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _4Inner {
            type Field = &'a mut T;

            fn view(&self, ( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make_tuples!(t, (_u, _v, _w, _x, t), (U, V, W, X, T));
make_tuples!(t, (_u, _v, _w, _x, t, _y), (U, V, W, X, T, Y));
make_tuples!(t, (_u, _v, _w, _x, t, _y, _z), (U, V, W, X, T, Y, Z));
make_tuples!(t, (_u, _v, _w, _x, t, _y, _z, _a), (U, V, W, X, T, Y, Z, A));
make_tuples!(
    t,
    (_u, _v, _w, _x, t, _y, _z, _a, _b),
    (U, V, W, X, T, Y, Z, A, B)
);
make_tuples!(
    t,
    (_u, _v, _w, _x, t, _y, _z, _a, _b, _c),
    (U, V, W, X, T, Y, Z, A, B, C)
);
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> LensView<[T; $n]> for _4Inner {
            type Field = T;

            fn view(&self, [ $($v,)* ]: [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<T> LensOver<[T; $n]> for _4Inner {
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

      impl<'a, T> LensView<&'a [T; $n]> for _4Inner {
            type Field = &'a T;

            fn view(&self, [ $($v,)* ]: &'a [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<'a, T> LensView<&'a mut [T; $n]> for _4Inner {
            type Field = &'a mut T;

            fn view(&self, [ $($v,)* ]: &'a mut [T; $n]) -> Self::Field {
                $f
            }
        }
    };
}

make_arrays!(t, 5, [_a, _b, _c, _d, t]);
make_arrays!(t, 6, [_a, _b, _c, _d, t, _e]);
make_arrays!(t, 7, [_a, _b, _c, _d, t, _e, _g]);
make_arrays!(t, 8, [_a, _b, _c, _d, t, _e, _g, _h]);
make_arrays!(t, 9, [_a, _b, _c, _d, t, _e, _g, _h, _i]);
