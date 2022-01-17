use bad_optics::prelude::*;

#[macro_use]
extern crate bad_optics;

// the Optics derive macro will implement lenses for every public field in the struct
// it makes a module named whatever the struct is called, but in lower case
// you can rename the generated module by adding `#[mod_name = "other_name"]` to the struct

#[derive(Optics, Clone, Debug)]
pub struct MyStruct {
    pub field1: String,
    pub field2: String,

    pub field3: u8,
    _field4: u8,
}

fn main() {
    let o = MyStruct {
        field1: "first field".to_string(),
        field2: "second field".to_string(),
        field3: 12,
        _field4: 1,
    };

    // we can manually get lenses for each field
    // note that it's a function that returns a lens
    //
    // these lenses work for `MyStruct` with `view` and `over`,
    // and for `&MyStruct` with `view`
    let field1 = <MyStruct as HasLens>::Lenses::field1();
    // short macro for convenience, expands to the line above
    let field2 = lens!(MyStruct::field2);

    // the lenses work normally as any other lens :)
    assert_eq!(field1(&o), "first field");
    assert_eq!(field2(&o), "second field");

    // we can get a vec with all the lenses that match a type
    let string_lenses = <MyStruct as HasLensOf<String>>::get();
    assert_eq!(string_lenses.len(), 2);

    // since _field4 is private, there's no lens for it
    let u8_lenses = <MyStruct as HasLensOf<u8>>::get();
    assert_eq!(u8_lenses.len(), 1);

    // short macro for convenience, expands to the line above
    let _u8_lenses = lenses!(MyStruct::u8);

    let mut o = o;
    for lens in string_lenses {
        o = lens(o, |s| s.to_ascii_uppercase());
    }
    dbg!(o);
}
