use bad_optics::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct MyStruct {
    hey: (u8, (u8, [Option<i32>; 5])),
}

fn main() {
    // make a lens that accesses `hey` in `MyStruct`
    let hey = lens(
        |hello: MyStruct| hello.hey,
        |mut hello: MyStruct, v| {
            hello.hey = v;
            hello
        },
    );

    // the thing we want to access
    let thing: (MyStruct, &'static str) = (
        MyStruct {
            hey: (1, (2, [None, Some(1), Some(2), None, Some(4)])),
        },
        "hello",
    );

    let array_lens = _0 // access the first element in the tuple
              + hey // access hey
              + _1 // access the second element in the tuple
              + _1; // access the second element in the tuple

    assert_eq!(array_lens(thing.clone()).len(), 5);

    let lens = array_lens
              + each // access each element of the [Option<i32>; 5] array
              + _Some; // access the ones that are Some;

    assert_eq!(lens(thing.clone()), vec![1, 2, 4]);

    assert_eq!(
        lens(thing, |v| v + 10),
        (
            MyStruct {
                hey: (1, (2, [None, Some(11), Some(12), None, Some(14)])),
            },
            "hello",
        )
    );
}
