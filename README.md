# Computus
[![Clippy Linting Result](https://clippy.bashy.io/github/dten/computus/master/badge.svg)](https://clippy.bashy.io/github/dten/computus/master/log)

Computus Easter calculation in Rust

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
computus = "^0.1"
```

and this to your crate root:

```rust
extern crate computus;
```

then you can find when Easter is for a particular year with:

```rust
// For Gregorian calendars
let (month, day) = computus::gregorian::month_day(2016);
// For Julian calendars
let (month, day) = computus::julian::month_day(2016);
```
