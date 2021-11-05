use bad_optics::lenses::*;

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
    assert_eq!(res, ((1, 3), 3));
}
