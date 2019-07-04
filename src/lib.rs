/*!
# Starts/Ends With Caseless

This crate provides the `StartsWithCaseless` trait and the `EndsWithCaseless` trait to extend types which implement `AsRef<str>` in order to do `starts_with` and `ends_with` case-insensitively.

## Examples

```rust,ignore
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::StartsWithCaseless;

assert_eq!(true, "foobar".starts_with_caseless_ascii("FoO"));

assert_eq!(true, "Maße 123".starts_with_caseless("MASSE"));
```

```rust,ignore
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaseless;

assert_eq!(true, "foobar".ends_with_caseless_ascii("BaR"));

assert_eq!(true, "123 Maße".ends_with_caseless("MASSE"));
```

## Multiple Prefixes or Suffixes

This crate also provides the `StartsWithCaselessMultiple` trait and the `EndsWithCaselessMultiple` trait to do `starts_with_caseless` and `ends_with_caseless` with multiple prefixes or suffixes.

### Example

```rust,ignore
extern crate starts_ends_with_caseless;

use starts_ends_with_caseless::EndsWithCaselessMultiple;

assert_eq!(Some(1), "photo.jpg".ends_with_caseless_ascii_multiple(&[".png", ".jpg", ".gif"]));
```

## No Std

This crate can work without std, but the `starts_with_caseless`, `ends_with_caseless`, `starts_with_caseless_multiple` and `ends_with_caseless_multiple` methods will be disabled at this stage as well.

Enable the feature **no_std** to compile this crate without std.

```toml
[dependencies.starts-ends-with-caseless]
version = "*"
features = ["no_std"]
```
*/

#![cfg_attr(feature = "no_std", no_std)]

mod starts_with_caseless;
mod ends_with_caseless;
mod starts_with_caseless_multiple;
mod ends_with_caseless_multiple;

pub use starts_with_caseless::StartsWithCaseless;
pub use ends_with_caseless::EndsWithCaseless;
pub use starts_with_caseless_multiple::StartsWithCaselessMultiple;
pub use ends_with_caseless_multiple::EndsWithCaselessMultiple;