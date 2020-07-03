extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaseless;

#[test]
fn starts_with_caseless_ascii() {
    assert_eq!(true, "foobar".starts_with_caseless_ascii("foo"));
    assert_eq!(true, "foobar".starts_with_caseless_ascii("FOO"));
    assert_eq!(true, "foobar".starts_with_caseless_ascii("FoO"));
    assert_eq!(true, "foobar".starts_with_caseless_ascii("FooBar"));
    assert_eq!(false, "foobar".starts_with_caseless_ascii("bar"));
    assert_eq!(false, "foobar".starts_with_caseless_ascii(" foobar"));
}

#[cfg(feature = "std")]
#[test]
fn starts_with_caseless() {
    assert_eq!(true, "Maße 123".starts_with_caseless("Maße"));
    assert_eq!(true, "Maße 123".starts_with_caseless("MASSE"));
    assert_eq!(false, "Maße 123".starts_with_caseless("messe"));
    assert_eq!(false, "Maße 123".starts_with_caseless("123"));
    assert_eq!(false, "Maße 123".starts_with_caseless(" Maße 123"));
}
