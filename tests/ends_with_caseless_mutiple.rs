#![cfg(not(feature = "no_std"))]

extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaselessMultiple;

#[test]
fn ends_with_caseless_ascii_multiple() {
    assert_eq!(true, "foobar".ends_with_caseless_ascii_multiple(&["foo", "bar"]));
    assert_eq!(true, "foobar".ends_with_caseless_ascii_multiple(&["foo", "FooBar"]));
    assert_eq!(false, "foobar".ends_with_caseless_ascii_multiple(&["foo", "ba"]));
}

#[test]
fn ends_with_caseless_multiple() {
    // TODO
}