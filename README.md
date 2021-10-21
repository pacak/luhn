Validates strings and computes check digits using the Luhn algorithm.


This is a fast, allocation and panic free crate for computing and validating
Luhn checksum for decimal and alpha numeric sequences. This is my crate. There are many like
it, but this one is mine.

It's not a great checksum, but it's used in a bunch of places:
- credit card numbers
- International Securities Identification Number (ISIN) codes
- International Mobile Equipment Identity (IMEI) codes
- Canadian Social Insurance Numbers

More information is available on [wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).

## Usage

Add `luhn3` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
luhn3 = "1.0"
```

Most of the CC numbers use Luhn checksum:

```rust
// Visa
luhn3::valid(b"4111111111111111"); // true

// MasterCard
luhn3::valid(b"5555555555554444"); // true

// Invalid Visa
luhn3::valid(b"4111111111111121"); // false
```

Library also allows to calculate a checksum if missing:

```rust
// Take off the checksum from American Express card number
let (&check, body) = b"378282246310005".split_last().unwrap();
// and recalculate it
luhn3::checksum(body); // Some(b'5')
```

This library provides two sets of operation: [`decimal`] and [`alphanum`].
- `decimal` operates on sequences composed of decimal numbers only, such
as credit card numbers or `IMEI` codes
- `alphanum` operates on sequences composed of decimal numbers and capital latin letters, such
  as `ISIN` or `NSIN`

## no_std

Crate doesn't use `std`

## Performance

```ignore
validate isin           time:   [13.136 ns 13.181 ns 13.230 ns]
validate visa           time:   [13.523 ns 13.572 ns 13.627 ns]
```
