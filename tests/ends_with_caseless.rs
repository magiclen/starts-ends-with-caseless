extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaseless;

#[test]
fn ends_with_caseless_ascii() {
    assert_eq!(true, "foobar".ends_with_caseless_ascii("bar"));
    assert_eq!(true, "foobar".ends_with_caseless_ascii("BAR"));
    assert_eq!(true, "foobar".ends_with_caseless_ascii("BaR"));
    assert_eq!(true, "foobar".ends_with_caseless_ascii("FooBar"));
    assert_eq!(false, "foobar".ends_with_caseless_ascii("foo"));
    assert_eq!(false, "foobar".ends_with_caseless_ascii(" foobar"));
}

#[cfg(not(feature = "no_std"))]
#[test]
fn ends_with_caseless() {
    assert_eq!(true, "123 Maße".ends_with_caseless("Maße"));
    assert_eq!(true, "123 Maße".ends_with_caseless("MASSE"));
    assert_eq!(false, "123 Maße".ends_with_caseless("messe"));
    assert_eq!(false, "123 Maße".ends_with_caseless("123"));
    assert_eq!(false, "123 Maße".ends_with_caseless(" Maße 123"));
}