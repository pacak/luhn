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

Library contains scalar implementations for both decimal and alhpanumeric inputs.
For validations there are variants that take a slice and variants that take an array.
The only difference is that compiler can optimize operations on array better so they
are slightly faster.


```ignore
validate isin           time:   [13.136 ns 13.181 ns 13.230 ns]
validate isin arr       time:   [9.5167 ns 9.5647 ns 9.6168 ns]
validate visa           time:   [8.3910 ns 8.4963 ns 8.6302 ns]
validate visa arr       time:   [5.3921 ns 5.4192 ns 5.4487 ns]
```

For non 64bit platforms implementation operating on alphanumeric input might perform
better.
