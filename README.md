Starts/Ends With Caseless
====================

[![Build Status](https://travis-ci.org/magiclen/starts-ends-with-caseless.svg?branch=master)](https://travis-ci.org/magiclen/starts-ends-with-caseless)

This crate provides the `StartsWithCaseless` trait and the `EndsWithCaseless` trait to extend types which implement `AsRef<str>` in order to do `starts_with` and `ends_with` case-insensitively.

## Examples

```rust
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaseless;

assert_eq!(true, "foobar".starts_with_caseless_ascii("FoO"));

assert_eq!(true, "Maße 123".starts_with_caseless("MASSE"));
```

```rust
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaseless;

assert_eq!(true, "foobar".ends_with_caseless_ascii("BaR"));

assert_eq!(true, "123 Maße".ends_with_caseless("MASSE"));
```

## Crates.io

https://crates.io/crates/starts-ends-with-caseless

## Documentation

https://docs.rs/starts-ends-with-caseless

## License

[MIT](LICENSE)