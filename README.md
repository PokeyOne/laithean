# Làithean

A Rust library for rendering dates in scottish gaelic.

Leabharlann Rust airson fòrmatadh ceann-làithean anns an Gàidhlig

## Example

```rust
use laithean::Date;
use laithean::month::Month;

fn main() {
    let date = Date::new(2022, Month::March, 0);

    assert_eq!("Dimàirt 1 Màrt 2022", date.scottish_gaelic(None));
    assert_eq!("Tuesday 1 March 2022", date.english(None));

    // The Date struct also implements the Display trait
    println!("'S e {date} a th' ann.");
}
```
