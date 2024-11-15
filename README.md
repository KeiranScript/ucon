# uCON

Unit conversion in Rust has never been easier.

## Supported Conversions

- Length
- Weight
- Temperature
- Volume
- Time
- Speed
- Area
- Data

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ucon = "0.1.0"
```
or add with cargo:

```shell
cargo add ucon
```

```rust
use ucon::Length;

fn main() {
    let meter = Length::Meter(1.0);
    let kilometer = meter.convert_to(&Length::Kilometer(0.0)).unwrap();
    println!("{} is {}", meter, kilometer);
}
```
