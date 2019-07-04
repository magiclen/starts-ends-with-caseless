extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaselessMultiple;

#[test]
fn starts_with_caseless_ascii_multiple() {
//    assert_eq!(Some(0), "foobar".starts_with_caseless_ascii_multiple(&["foo", "bar"]));
    assert_eq!(Some(1), "foobar".starts_with_caseless_ascii_multiple(&["bar", "FooBar"]));
//    assert_eq!(None, "foobar".starts_with_caseless_ascii_multiple(&["bar", "oo"]));
}

#[cfg(not(feature = "no_std"))]
#[test]
fn starts_with_caseless_multiple() {
    // TODO
}