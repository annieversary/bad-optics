use crate::{
    lenses::{LensOver, LensView},
    Optics, OpticsTrait,
};

#[derive(Clone, Copy)]
pub struct _0Inner;
pub const _0: Optics<_0Inner> = Optics(_0Inner);
impl OpticsTrait for _0Inner {}

macro_rules! make_tuples {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
      impl< $($t,)* > LensView<( $($t,)* )> for _0Inner {
            type Field = T;

            fn view(&self, ( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl< $($t,)* > LensOver<( $($t,)* )> for _0Inner {
            fn over<F>(
                &self,
                mut tup: ($($t,)*),
                f: F
            ) -> ( $($t,)* )
            where
              F: FnOnce(Self::Field) -> Self::Field
            {
                tup.0 = f(tup.0);
                tup
            }
        }

        impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _0Inner {
            type Field = &'a T;

            fn view(&self, ( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _0Inner {
            type Field = &'a mut T;

            fn view(&self, ( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make_tuples!(t, (t, _u), (T, U));
make_tuples!(t, (t, _u, _v), (T, U, V));
make_tuples!(t, (t, _u, _v, _w), (T, U, V, W));
make_tuples!(t, (t, _u, _v, _w, _x), (T, U, V, W, X));
make_tuples!(t, (t, _u, _v, _w, _x, _y), (T, U, V, W, X, Y));
make_tuples!(t, (t, _u, _v, _w, _x, _y, _z), (T, U, V, W, X, Y, Z));
// not doing more cause i'm lazy, open a pr if you need more :)

macro_rules! make_arrays {
    ($f:ident, $n:expr, [$( $v:ident ),*]) => {
      impl<T> LensView<[T; $n]> for _0Inner {
            type Field = T;

            fn view(&self, [ $($v,)* ]: [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<T> LensOver<[T; $n]> for _0Inner {
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

      impl<'a, T> LensView<&'a [T; $n]> for _0Inner {
            type Field = &'a T;

            fn view(&self, [ $($v,)* ]: &'a [T; $n]) -> Self::Field {
                $f
            }
        }
      impl<'a, T> LensView<&'a mut [T; $n]> for _0Inner {
            type Field = &'a mut T;

            fn view(&self, [ $($v,)* ]: &'a mut [T; $n]) -> Self::Field {
                $f
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
