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

## Multiple Prefixes or Suffixes

This crate also provides the `StartsWithCaselessMultiple` trait and the `EndsWithCaselessMultiple` trait to do `starts_with_caseless` and `ends_with_caseless` with multiple prefixes or suffixes.

### Example

```rust
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaselessMultiple;

assert_eq!(Some(1), "photo.jpg".ends_with_caseless_ascii_multiple(&[".png", ".jpg", ".gif"]));
```

## No Std

Disable the default features to compile this crate without std. But the `starts_with_caseless`, `ends_with_caseless`, `starts_with_caseless_multiple` and `ends_with_caseless_multiple` methods will be disabled at this stage as well.

```toml
[dependencies.html-minifier]
version = "*"
default-features = false
```

## Crates.io

https://crates.io/crates/starts-ends-with-caseless

## Documentation

https://docs.rs/starts-ends-with-caseless

## License

[MIT](LICENSE)