#![cfg(not(feature = "no_std"))]

extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaselessMultiple;

#[test]
fn starts_with_caseless_ascii_multiple() {
    assert_eq!(true, "foobar".starts_with_caseless_ascii_multiple(&["foo", "bar"]));
    assert_eq!(true, "foobar".starts_with_caseless_ascii_multiple(&["bar", "FooBar"]));
    assert_eq!(false, "foobar".starts_with_caseless_ascii_multiple(&["bar", "oo"]));
}

#[test]
fn starts_with_caseless_multiple() {
    // TODO
}