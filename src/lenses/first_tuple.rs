use crate::{LensOver, LensView};

pub struct _0;

macro_rules! make {
    ($f:ident, ( $( $v:ident ),* ), ( $( $t:ident ),* ) ) => {
        impl< $($t,)* > LensView<( $($t,)* )> for _0 {
            type Field = T;

            fn view(( $($v,)* ): ($($t,)*)) -> Self::Field {
                $f
            }
        }

        impl< $($t,)* > LensOver<( $($t,)* )> for _0 {
            fn over(
                mut tup: ($($t,)*),
                f: &dyn Fn(Self::Field) -> Self::Field
            ) -> ( $($t,)* ) {
                tup.0 = f(tup.0);
                tup
            }
        }
        impl<'a, $($t,)* > LensView<&'a ( $($t,)* )> for _0 {
            type Field = &'a T;

            fn view(( $($v,)* ): &'a ($($t,)*)) -> Self::Field {
                $f
            }
        }
        impl<'a, $($t,)* > LensView<&'a mut ( $($t,)* )> for _0 {
            type Field = &'a mut T;

            fn view(( $($v,)* ): &'a mut ($($t,)*)) -> Self::Field {
                $f
            }
        }
    };
}

make!(t, (t, _u), (T, U));
make!(t, (t, _u, _v), (T, U, V));
make!(t, (t, _u, _v, _w), (T, U, V, W));
make!(t, (t, _u, _v, _w, _x), (T, U, V, W, X));
make!(t, (t, _u, _v, _w, _x, _y), (T, U, V, W, X, Y));
make!(t, (t, _u, _v, _w, _x, _y, _z), (T, U, V, W, X, Y, Z));
// not doing more cause i'm lazy, open a pr if you need more :)
