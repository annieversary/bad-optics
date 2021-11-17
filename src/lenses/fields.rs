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

#[macro_export]
macro_rules! field_lens_with_ref {
    ($type:ident, $field:ident) => {
        $crate::lenses::lens_with_ref(
            |v: $type| v.$field,
            |mut u: $type, v| {
                u.$field = v;
                u
            },
            |v: &$type| v.$field.clone(),
        )
    };
}
