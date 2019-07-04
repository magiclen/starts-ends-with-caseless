extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaselessMultiple;

#[test]
fn ends_with_caseless_ascii_multiple() {
    assert_eq!(Some(1), "foobar".ends_with_caseless_ascii_multiple(&["foo", "bar"]));
    assert_eq!(Some(1), "foobar".ends_with_caseless_ascii_multiple(&["foo", "FooBar"]));
    assert_eq!(None, "foobar".ends_with_caseless_ascii_multiple(&["foo", "ba"]));
}

#[cfg(not(feature = "no_std"))]
#[test]
fn ends_with_caseless_multiple() {
    // TODO
}