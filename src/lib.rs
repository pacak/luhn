#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Digit mixer for one symbol at a time consuming.
///
/// This structure allows to calculate Luhn chechsums for strings with additional formatting
/// present without having to reallocate a formatting-less slice. See [`Mixer::push`] for more
/// details.
///
/// # Examples
/// ```rust
///    use luhn3::Mixer;
///    let input = "4111 1111 1111 1111";
///    let mut m = Mixer::default();
///    for c in input.as_bytes() {
///        if (b'0'..=b'9').contains(c) {
///            m.push(c - b'0');
///        }
///    }
///    assert!(m.valid());
/// ```
///
/// Luhn algorithm calls for a sum of digits in odd and even places starting
/// from the end of the stream with additional transmogrification applied to
/// digits on even (if calculating missing check digit) or odd (if checking the
/// check digit) places starting from the right. Since odd and even position
/// are defined in terms of the right most digit [`Mixer`] tries to keep
/// enough information available to be able to perform the final calculation
/// for both even and odd sized transmogrified strings.
#[derive(Default)]
pub struct Mixer(Blob, Blob);

impl Mixer {
    /// Add a new digit to current checksum computation
    ///
    /// Input must be decimal digit in `0..9` range inclusive so for `'1'` the correct
    /// value to push is `1`.  For alphanumeric values two pushes are required: `'A'`
    /// represents number `10` and should be pushed as `1` followed by `0`.
    ///
    /// # Panics
    /// Function contains [debug_assert] to ensure correct input
    ///
    pub fn push(&mut self, digit: u8) {
        debug_assert!(digit < 10);
        if digit >= 5 {
            self.0.five_or_higher += 1;
        }
        self.0.sum += usize::from(digit);

        // swap blobs
        let t = self.0;
        self.0 = self.1;
        self.1 = t;
    }

    pub fn valid(&self) -> bool {
        (self.0.sum * 2 - self.0.five_or_higher * 9 + self.1.sum) % 10 == 0
    }

    pub fn checksum(&self) -> u8 {
        let checksum = self.1.sum * 2 - self.1.five_or_higher * 9 + self.0.sum;
        b'0' + ((10 - (checksum % 10)) % 10) as u8
    }
}

#[derive(Default, Copy, Clone)]
struct Blob {
    sum: usize,
    five_or_higher: usize,
}

fn fold_decimal(raw: &[u8]) -> Option<Mixer> {
    let mut mixer = Mixer::default();
    for c in raw.iter().copied() {
        match c {
            b'0'..=b'9' => mixer.push(c - b'0'),
            _ => return None,
        }
    }
    Some(mixer)
}

fn fold_base36(raw: &[u8]) -> Option<Mixer> {
    let mut mixer = Mixer::default();
    for c in raw.iter().copied() {
        match c {
            b'0'..=b'9' => mixer.push(c - b'0'),
            b'A'..=b'Z' => {
                let b36 = c - b'A' + 10;
                mixer.push(b36 / 10);
                mixer.push(b36 % 10);
            }
            _ => return None,
        }
    }
    Some(mixer)
}

pub mod decimal {
    //! # Operations on decimal only Luhn checksums
    //!
    //! A restricted version of the algorithm that only accepts decimal digits.
    //!
    //! ```
    //! use luhn3::decimal as luhn;
    //! // test Visa card number is valid
    //! assert!(luhn3::valid(b"4012888888881881"));
    //! ```
    //!
    //! See also [alphanum][crate::alphanum]
    use crate::*;

    /// Validate a check digit using Luhn algorithm
    ///
    /// Takes a slice of ASCII bytes and checks if the last byte is a valid Luhn checksum digit.
    /// Will return `false` if checksum digit valid but input is not a decimal only - for example
    /// an ISIN code. It is safe to pass non ASCII sequences of bytes.
    ///
    ///
    /// # Usage
    ///
    /// ```
    /// use luhn3::decimal::valid;
    ///
    /// // test Visa card is valid
    /// assert!(valid(b"4012888888881881"));
    ///
    /// // Microsoft's ISIN contains a valid checksum but it's
    /// // not a decimal
    /// assert!(!valid(b"US5949181045"));
    ///
    /// // Not a valid sequence
    /// let banana = String::from("banana");
    /// assert!(!valid(banana.as_bytes()));
    ///
    /// // Even less valid sequence
    /// let noms = "口水鸡";
    /// assert!(!valid(noms.as_bytes()));
    /// ```
    pub fn valid(ascii: &[u8]) -> bool {
        match fold_decimal(ascii) {
            Some(mixer) => mixer.valid(),
            None => false,
        }
    }

    /// Try to compute a checksum for a sequence of ASCII bytes
    ///
    /// If input contains only bytes in `b'0'..b'9'` range output
    /// is guaranteed to be a byte in `b'0'..=b'9'` range or None otherwise.
    /// ```
    /// use luhn3::decimal::checksum;
    ///
    /// // Can calculate a checksum for a string with decimal digits
    /// assert_eq!(Some(b'1'), checksum(b"401288888888188"));
    ///
    /// // Can't calculate a checksum for Microsoft's ISIN since
    /// // not a decimal sequence
    /// assert_eq!(None, checksum(b"US594918104"));
    ///
    /// // Not a valid sequence
    /// let banana = String::from("banana");
    /// assert_eq!(None, checksum(banana.as_bytes()));
    ///
    /// // Even less valid sequence
    /// let noms = "口水鸡";
    /// assert_eq!(None, checksum(noms.as_bytes()));
    /// ```
    pub fn checksum(ascii: &[u8]) -> Option<u8> {
        Some(fold_decimal(ascii)?.checksum())
    }
}

pub mod alphanum {
    //! # Operations on alphanumeric Luhn checksums
    //!
    //! This version of the algorithm can operate on decimal digits and capital ASCII letters.
    //!
    //! # Usage
    //!
    //! ```
    //! // Microsoft's ISIN is valid
    //! assert!(luhn3::valid(b"US5949181045"));
    //! ```
    //!
    //! See also [decimal][crate::decimal]
    use crate::*;

    /// Validate a check digit using Luhn algorithm
    ///
    /// ```
    /// use luhn3::alphanum::valid;
    ///
    /// // Microsoft's ISIN is valid
    /// assert!(valid(b"US5949181045"));
    ///
    /// // Not a valid sequence
    /// let banana = String::from("banana");
    /// assert!(!valid(banana.as_bytes()));
    ///
    /// // Even less valid sequence
    /// let noms = "口水鸡";
    /// assert!(!valid(noms.as_bytes()));
    /// ```
    pub fn valid(ascii: &[u8]) -> bool {
        match fold_base36(ascii) {
            Some(mixer) => mixer.valid(),
            None => false,
        }
    }

    /// Try to compute a check digit for a sequence of ASCII bytes
    ///
    /// If input contains only bytes in `b'0'..b'9' | b'A'..b'Z'` range output
    /// is guaranteed to be a byte in `b'0'..=b'9'` range or None otherwise.
    /// ```
    /// use luhn3::alphanum::checksum;
    ///
    /// // Can calculate a checksum for a string with decimal digits
    /// assert_eq!(Some(b'1'), checksum(b"401288888888188"));
    ///
    /// // Can calculate a checksum for Microsoft's ISIN
    /// assert_eq!(Some(b'5'), checksum(b"US594918104"));
    ///
    /// // Not a valid sequence
    /// let banana = String::from("banana");
    /// assert_eq!(None, checksum(banana.as_bytes()));
    ///
    /// // Even less valid sequence
    /// let noms = "口水鸡";
    /// assert_eq!(None, checksum(noms.as_bytes()));
    /// ```
    pub fn checksum(ascii: &[u8]) -> Option<u8> {
        Some(fold_base36(ascii)?.checksum())
    }
}

pub use crate::alphanum::*;

#[cfg(test)]
mod test {
    const DECIMAL_LUHN_SAMPLES: &'static [&str] = &[
        // test cc numbers
        "378282246310005",  // American Express
        "371449635398431",  // American Express
        "378734493671000",  // American Express Corporate
        "5610591081018250", // Australian BankCard
        "30569309025904",   // Diners Club
        "38520000023237",   // Diners Club
        "6011111111111117", // Discover
        "6011000990139424", // Discover
        "3530111333300000", // JCB
        "3566002020360505", // JCB
        "5555555555554444", // MasterCard
        "5105105105105100", // MasterCard
        "4111111111111111", // Visa
        "4012888888881881", // Visa
        "4222222222222",    // Visa
        "5019717010103742", // Dankort (PBS)
        "6331101999990016", // Switch/Solo (Paymentech)
        // random IMEI
        "358771054102508", // Apple iPad Air (A1475)
        "867103029110602", // HUAWEI G610-U20
        "358625057927511", // Cinterion PHS8-P
        "359513063006075", // Samsung GT-I9300I
        "351813076684290", // Samsung SM-G531H/DS
    ];

    fn change_digit(digit: u8) -> u8 {
        if digit == b'9' {
            b'0'
        } else {
            digit + 1
        }
    }

    #[test]
    fn test_ae_checksum() {
        let (&check, body) = b"378282246310005".split_last().unwrap();
        assert_eq!(Some(check), crate::decimal::checksum(body));
    }

    #[test]
    fn test_decimal_luhn_checksum() {
        for sample in DECIMAL_LUHN_SAMPLES {
            // number is valid as is valid
            assert!(crate::decimal::valid(sample.as_bytes()));

            // luhn checksum detects a single changed digit
            let mut s = Vec::from(*sample);
            s[3] = change_digit(s[3]);
            assert!(!crate::decimal::valid(&s));

            // luhn checksum also detects two digit swap
            let mut s = Vec::from(*sample);
            if s[3] != s[4] {
                s.swap(3, 4);
                assert!(!crate::decimal::valid(&s));
            }

            // last digit is it's luhn checksum
            let (checksum, body) = sample.as_bytes().split_last().unwrap();
            assert_eq!(Some(*checksum), crate::decimal::checksum(body));

            // and finally only decimal numbers are accepted
            let mut s = Vec::from(*sample);
            s[3] = b'x';
            assert!(!crate::decimal::valid(&s));
        }
    }

    const ALPHANUM_LUHN_SAMPLES: &'static [&str] = &[
        "US5949181045", // Microsoft
        "US38259P5089", // Google
        "US0378331005", // Apple
        "BMG491BT1088", // Invesco
        "IE00B4BNMY34", // Accenture
        "US0231351067", // Amazon
        "US64110L1061", // Netflix
        "US30303M1027", // Facebook
        "CH0031240127", // BMW Australia
        "CA9861913023", // Yorbeau Res
        // various Kospi200 options with all possible checksum digits
        "KR4101R60000",
        "KR4201QB2551",
        "KR4201RC3102",
        "KR4201Q92623",
        "KR4205QB2904",
        "KR4301R12825",
        "KR4301QC2906",
        "KR4205Q92327",
        "KR4301QB3228",
        "KR4301Q93579",
    ];

    #[test]
    fn test_alphanum_luhn_samples() {
        for sample in ALPHANUM_LUHN_SAMPLES {
            // number is valid as is valid
            assert!(crate::alphanum::valid(sample.as_bytes()));

            // luhn checksum detects a single changed digit
            let mut s = Vec::from(*sample);
            s[3] = change_digit(s[3]);
            assert!(!crate::alphanum::valid(&s));

            // last digit is it's luhn checksum
            let (checksum, body) = sample.as_bytes().split_last().unwrap();
            assert_eq!(Some(*checksum), crate::alphanum::checksum(body));

            // and finally only alphanum numbers are accepted
            let mut s = Vec::from(*sample);
            s[3] = b'x';
            assert!(!crate::alphanum::valid(&s));
        }
    }
}
