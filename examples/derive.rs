use bad_optics::prelude::Optics;

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
    let field1 = mystruct::field1();
    let field2 = mystruct::field2();

    // the lenses work normally as any other lens :)
    assert_eq!(field1(&o), "first field");
    assert_eq!(field2(&o), "second field");

    // we can get a vec with all the lenses that match a type
    let string_lenses = mystruct::Lenses::<String>::get();
    assert_eq!(string_lenses.len(), 2);

    // since _field4 is private, there's no lens for it
    let vec_string_lenses = mystruct::Lenses::<u8>::get();
    assert_eq!(vec_string_lenses.len(), 1);

    let mut o = o;
    for lens in string_lenses {
        o = lens(o, |s| s.to_ascii_uppercase());
    }
    dbg!(o);
}
