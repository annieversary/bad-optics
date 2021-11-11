use bad_optics::lenses::*;

#[derive(Debug, PartialEq, Clone)]
struct MyStruct {
    hey: (u8, (u8, i32)),
}

fn main() {
    // make a lens for Hello
    let hey = lens(
        |hello: MyStruct| hello.hey,
        |mut hello: MyStruct, v| {
            hello.hey = v;
            hello
        },
    );

    let my_struct = MyStruct { hey: (1, (2, -3)) };

    // the thing we want to access
    let thing = (my_struct, "hello");

    // a lens that targets the -3 inside my_struct
    let lens_that_targets_the_i32 = _0 + hey + _1 + _1;

    assert_eq!(lens_that_targets_the_i32(thing), -3);
}
