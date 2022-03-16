# Làithean

A Rust library for rendering dates in scottish gaelic.

## Example

```rust
use laithean::Date;
use laithean::month::Month;

fn main() {
    let date = Date::new(2022, Month::March, 0);

    assert_eq!("Dimàirt 1 Màrt 2022", date.scottish_gaelic(None));
}
```
