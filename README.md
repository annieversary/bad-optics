# bad optics

ergonomic lenses in rust

`bad-optics` implements the haskell concept of lenses, prisms, and traversals in rust

it does *not* implement the operators, as it's not really a thing we can do in rust

does bringing lenses into rust actually make sense? probably not, but it was fun to implement

## example

```rust
use bad_optics::{
    lenses::{over, set},
    prelude::*,
};

fn main() {
    let a = ((1, 2), 3);

    // use view to access inside the tuple
    let res = view(_0, a);
    assert_eq!(res, (1, 2));

    let res = view(_1, a);
    assert_eq!(res, 3);

    // you can combine lenses
    let lens = _0 + _1;

    // use the view function to access
    let res = view(lens, a);
    assert_eq!(res, 2);

    // you can also call the lens as a function
    let res = lens(a);
    assert_eq!(res, 2);

    // call the over function to modify the value
    let a = over(lens, a, |v| v + 1);
    assert_eq!(a, ((1, 3), 3));

    // call the set function to set the value
    let a = set(lens, a, 5);
    assert_eq!(a, ((1, 5), 3));

    // you can also call the lens as a function to modify the value
    let res = lens(a, |v| v + 1);
    assert_eq!(res, ((1, 6), 3));
}
```

## how to use

bad-optics provides some of the lenses, prisms, and traversals defined in `lens`. i'm still trying to add more, so if there's one you need and it's missing from here, feel free to open a PR

if you don't know how lenses work, this is not really gonna be a tutorial, you should read [this](https://hackage.haskell.org/package/lens-tutorial-1.0.3/docs/Control-Lens-Tutorial.html) first instead. the general idea is that they are first-class getters and setters
