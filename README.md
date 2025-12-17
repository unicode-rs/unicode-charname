# unicode-charname

[![Docs](https://docs.rs/unicode-charname/badge.svg)](https://docs.rs/unicode-charname/)

Unicode character name properties
as described in
[Unicode Standard Annex #44](http://www.unicode.org/reports/tr44/).

```rust
extern crate unicode_charname;

use unicode_charname::CharName;

fn main() {
    assert_eq!('A'.char_name().unwrap_or_default().to_string(),
                "LATIN CAPITAL LETTER A");
}
```

## crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
unicode-charname = "0.1"
```

## `no_std` + `alloc` support

This crate is completely `no_std` + `alloc` compatible. This can be enabled by disabling the `std` feature, i.e. specifying `default-features = false` for this crate on your `Cargo.toml`.
