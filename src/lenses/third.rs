use crate::lenses::{Lens, LensOver, LensView};

#[derive(Clone, Copy)]
pub struct _2Inner;
pub const _2: Lens<_2Inner> = Lens(_2Inner);

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl< $($t,)* > LensView<( $($t,)* )> for _2Inner {
            type Field = T;

            fn view(&self, ( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl< $($t,)* > LensOver<( $($t,)* )> for _2Inner {
            fn over<F>(
                &self,
                mut tup: ($($t,)*),
                f: F
            ) -> ( $($t,)* )
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
                tup.2 = f(tup.2);
                tup
            }
        }

        impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _2Inner {
            type Field = &'a T;

            fn view(&self, ( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _2Inner {
            type Field = &'a mut T;

            fn view(&self, ( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make_tuples!(t, (_u, _v, t), (U, V, T));
make_tuples!(t, (_u, _v, t, _w), (U, V, T, W));
make_tuples!(t, (_u, _v, t, _w, _x), (U, V, T, W, X));
make_tuples!(t, (_u, _v, t, _w, _x, _y), (U, V, T, W, X, Y));
make_tuples!(t, (_u, _v, t, _w, _x, _y, _z), (U, V, T, W, X, Y, Z));
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> LensView<[T; $n]> for _2Inner {
            type Field = T;

            fn view(&self, [ $($v,)* ]: [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<T> LensOver<[T; $n]> for _2Inner {
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

      impl<'a, T> LensView<&'a [T; $n]> for _2Inner {
            type Field = &'a T;

            fn view(&self, [ $($v,)* ]: &'a [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<'a, T> LensView<&'a mut [T; $n]> for _2Inner {
            type Field = &'a mut T;

            fn view(&self, [ $($v,)* ]: &'a mut [T; $n]) -> Self::Field {
                $f
            }
        }
    };
}

make_arrays!(t, 3, [_a, _b, t]);
make_arrays!(t, 4, [_a, _b, t, _c]);
make_arrays!(t, 5, [_a, _b, t, _c, _d]);
make_arrays!(t, 6, [_a, _b, t, _c, _d, _e]);
make_arrays!(t, 7, [_a, _b, t, _c, _d, _e, _g]);
