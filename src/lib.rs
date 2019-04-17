/*!
# Starts/Ends With Caseless

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
*/

mod starts_with_caseless;
mod ends_with_caseless;

pub use starts_with_caseless::StartsWithCaseless;
pub use ends_with_caseless::EndsWithCaseless;