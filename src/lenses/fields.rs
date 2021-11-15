#[macro_export]
macro_rules! field_lens {
    ($type:ident, $field:ident) => {
        $crate::lenses::lens(
            |v: $type| v.$field,
            |mut u: $type, v| {
                u.$field = v;
                u
            },
        )
    };
}
