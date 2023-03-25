# Computus
[![crates.io](https://img.shields.io/crates/v/computus.svg)](https://crates.io/crates/computus)
[![Documentation](https://docs.rs/computus/badge.svg)](https://docs.rs/computus)

Computus Easter calculation in Rust

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
computus = "1.1.0"
```

You can find when Easter Sunday is for a particular year with:

```rust
// For Gregorian calendars
let easter = computus::gregorian(2016).unwrap();
assert_eq!((easter.month, easter.day), (3, 27));
// For Julian calendars
let easter = computus::julian(2016).unwrap();
assert_eq!((easter.month, easter.day), (4, 18));
// With `chrono` feature
#[cfg(feature = "chrono")] {
    use chrono::Datelike;
    let easter = computus::gregorian_naive(2023).unwrap();
    assert_eq!((easter.month(), easter.day()), (4, 9));
}
```