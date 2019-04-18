Starts/Ends With Caseless
====================

[![Build Status](https://travis-ci.org/magiclen/starts-ends-with-caseless.svg?branch=master)](https://travis-ci.org/magiclen/starts-ends-with-caseless)
[![Build status](https://ci.appveyor.com/api/projects/status/0yh87l4emfadyc23/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/starts-ends-with-caseless/branch/master)

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

This crates also provides the `StartsWithCaselessMultiple` trait and the `EndsWithCaselessMultiple` trait to do `starts_with_caseless` and `ends_with_caseless` with multiple prefixes or suffixes.

### Example

```rust
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaselessMultiple;

assert_eq!(true, "foobar".starts_with_caseless_ascii_multiple(&["foo", "bar"]));
```

## No Std

This crate can work without std, but the `starts_with_caseless` method and the `ends_with_caseless` method will be disabled at this stage as well as the `StartsWithCaselessMultiple` trait and the `EndsWithCaselessMultiple` trait.

Enable the feature **no_std** to compile this crate without std.

```toml
[dependencies.starts-ends-with-caseless]
version = "*"
features = ["no_std"]
```

## Crates.io

https://crates.io/crates/starts-ends-with-caseless

## Documentation

https://docs.rs/starts-ends-with-caseless

## License

[MIT](LICENSE)