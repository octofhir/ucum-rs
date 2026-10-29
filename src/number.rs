//! Exact decimal numbers of any magnitude for unit factors.
//!
//! A `Number` is `mantissa × 10^exponent` with a decimal mantissa of up to 28 significant
//! digits. Unlike a plain `Decimal`, whose range is about 1e-28 to 7.9e28, it holds the
//! factors of every UCUM unit (`[m_e]` is 9.1093837139e-28 g, `Ym3` is 1e72 m3) without
//! losing digits, and unlike an `f64` it keeps decimal factors exact (`[in_i]` is 2.54 cm,
//! not 2.5400000000000000355 cm).

use core::cmp::Ordering;
use core::fmt;
use rust_decimal::Decimal;

/// A decimal number `mantissa × 10^exponent`, normalized so that the mantissa is zero or
/// has an absolute value in [1, 10). Results of arithmetic are rounded to 28 significant
/// digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Number {
    mantissa: Decimal,
    exponent: i32,
}

impl Number {
    /// Zero.
    pub const ZERO: Self = Self {
        mantissa: Decimal::ZERO,
        exponent: 0,
    };

    /// One.
    pub const ONE: Self = Self {
        mantissa: Decimal::ONE,
        exponent: 0,
    };

    /// A number from the parts of its normalized mantissa (see `Decimal::from_parts`) and
    /// its exponent, as the generated registry writes it. The mantissa must be zero or have
    /// an absolute value in [1, 10).
    #[doc(hidden)]
    pub const fn from_parts(
        lo: u32,
        mid: u32,
        hi: u32,
        negative: bool,
        scale: u32,
        exponent: i32,
    ) -> Self {
        Self {
            mantissa: Decimal::from_parts(lo, mid, hi, negative, scale),
            exponent,
        }
    }

    /// The parts that `from_parts` takes, for code generation.
    #[doc(hidden)]
    pub fn to_parts(self) -> (u32, u32, u32, bool, u32, i32) {
        let unpacked = self.mantissa.unpack();
        (
            unpacked.lo,
            unpacked.mid,
            unpacked.hi,
            unpacked.negative,
            unpacked.scale,
            self.exponent,
        )
    }

    /// `mantissa × 10^exponent`, normalized.
    fn new(mantissa: Decimal, exponent: i32) -> Option<Self> {
        if mantissa.is_zero() {
            return Some(Self::ZERO);
        }
        let mantissa = mantissa.normalize();
        let digits = digit_count(mantissa.mantissa().unsigned_abs());
        // `digits - 1` is at most 28: a mantissa has at most 96 bits
        let mut normalized = Decimal::from_i128_with_scale(mantissa.mantissa(), digits - 1);
        let mut shift = digits as i64 - 1 - i64::from(mantissa.scale());
        // A mantissa of 29 digits is rounded to 28, as `parse` does
        if digits > 28 {
            normalized = normalized
                .round_dp_with_strategy(27, rust_decimal::RoundingStrategy::MidpointAwayFromZero);
            if normalized.abs() >= Decimal::TEN {
                normalized /= Decimal::TEN;
                shift += 1;
            }
        }
        let mantissa = normalized.normalize();
        let exponent = i32::try_from(i64::from(exponent) + shift).ok()?;
        // So that the exponent of `to_integer_and_exponent` fits too
        exponent.checked_sub(mantissa.scale() as i32)?;
        Some(Self { mantissa, exponent })
    }

    /// An integer.
    pub fn from_i64(value: i64) -> Self {
        Self::new(Decimal::from(value), 0).expect("an i64 is in range")
    }

    /// The exact value of a decimal written as digits with an optional fraction and
    /// exponent: "2.54", "6.02214076e23", "1e-7", "-273.15". Digits beyond 28 significant
    /// ones are rounded.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        let (body, exponent) = match text.split_once(['e', 'E']) {
            Some((body, exponent)) => (body, exponent.parse::<i32>().ok()?),
            None => (text, 0),
        };
        let (negative, body) = match body.as_bytes().first()? {
            b'-' => (true, &body[1..]),
            b'+' => (false, &body[1..]),
            _ => (false, body),
        };
        let (integer, fraction) = body.split_once('.').unwrap_or((body, ""));
        if integer.is_empty() && fraction.is_empty()
            || !integer
                .bytes()
                .chain(fraction.bytes())
                .all(|b| b.is_ascii_digit())
        {
            return None;
        }
        // Significant digits and the power of ten of the last one
        let all = integer.bytes().chain(fraction.bytes());
        let leading_zeros = all.clone().take_while(|&b| b == b'0').count();
        let significant: alloc::vec::Vec<u8> = all.skip(leading_zeros).collect();
        if significant.is_empty() {
            return Some(Self::ZERO);
        }
        // value = 0.d1d2d3... × 10^point
        let point = integer.len() as i64 - leading_zeros as i64;
        // 28 significant digits, rounded half away from zero on the first dropped one
        let kept = &significant[..significant.len().min(28)];
        let mut mantissa: i128 = 0;
        for &b in kept {
            mantissa = mantissa * 10 + i128::from(b - b'0');
        }
        if significant.get(kept.len()).is_some_and(|&b| b >= b'5') {
            mantissa += 1;
        }
        if negative {
            mantissa = -mantissa;
        }
        let decimal = Decimal::from_i128_with_scale(mantissa, (kept.len() - 1) as u32);
        let exponent = i64::from(exponent) + point - 1;
        Self::new(decimal, i32::try_from(exponent).ok()?)
    }

    /// The exact decimal value of an `f64`, read from its shortest representation (0.1 is
    /// 0.1, not 0.1000000000000000055511151231257827). `None` for an infinite or NaN value.
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let mut buffer = Buffer::new();
        fmt::write(&mut buffer, format_args!("{value:e}")).ok()?;
        Self::parse(buffer.as_str())
    }

    /// The nearest `f64`, rounded once. Infinite when the number is beyond the `f64` range.
    pub fn to_f64(self) -> f64 {
        if self.mantissa.is_zero() {
            return 0.0;
        }
        let mut buffer = Buffer::new();
        fmt::write(
            &mut buffer,
            format_args!("{}e{}", self.mantissa, self.exponent),
        )
        .expect("a number fits the buffer");
        buffer.as_str().parse().expect("a number parses as f64")
    }

    /// Whether the number is zero.
    pub fn is_zero(self) -> bool {
        self.mantissa.is_zero()
    }

    /// Whether the number is below zero.
    pub fn is_negative(self) -> bool {
        self.mantissa.is_sign_negative() && !self.mantissa.is_zero()
    }

    /// The absolute value.
    pub fn abs(self) -> Self {
        Self {
            mantissa: self.mantissa.abs(),
            exponent: self.exponent,
        }
    }

    /// `self × other`, or `None` when the exponent overflows.
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        if self.is_zero() || other.is_zero() {
            return Some(Self::ZERO);
        }
        Self::new(
            self.mantissa.checked_mul(other.mantissa)?,
            self.exponent.checked_add(other.exponent)?,
        )
    }

    /// `self / other`, or `None` for a division by zero or when the exponent overflows.
    pub fn checked_div(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            return None;
        }
        if self.is_zero() {
            return Some(Self::ZERO);
        }
        Self::new(
            self.mantissa.checked_div(other.mantissa)?,
            self.exponent.checked_sub(other.exponent)?,
        )
    }

    /// `self + other`, to the 28 significant digits of the bigger term. A term more than 28
    /// orders of magnitude below the other one does not change it.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            return Some(self);
        }
        if self.is_zero() {
            return Some(other);
        }
        let (big, small) = if self.exponent >= other.exponent {
            (self, other)
        } else {
            (other, self)
        };
        let shift = big.exponent.abs_diff(small.exponent);
        if shift > 28 {
            return Some(big);
        }
        let aligned = small.mantissa.checked_mul(Decimal::new(1, shift))?;
        // Digits below the 28 significant ones of the bigger term are rounding noise: 32
        // [degF] is 0 Cel, not -2.4e-26
        let sum = big
            .mantissa
            .checked_add(aligned)?
            .round_dp_with_strategy(27, rust_decimal::RoundingStrategy::MidpointAwayFromZero);
        Self::new(sum, big.exponent)
    }

    /// `self - other`.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_add(-other)
    }

    /// `self^exp`, by squaring, or `None` when the exponent overflows or for a negative
    /// power of zero.
    pub fn checked_powi(self, exp: i32) -> Option<Self> {
        let mut result = Self::ONE;
        let mut square = self;
        let mut n = exp.unsigned_abs();
        while n > 0 {
            if n & 1 == 1 {
                result = result.checked_mul(square)?;
            }
            n >>= 1;
            if n > 0 {
                square = square.checked_mul(square)?;
            }
        }
        if exp < 0 {
            Self::ONE.checked_div(result)
        } else {
            Some(result)
        }
    }

    /// `(digits, exp)` such that the number is `digits × 10^exp`, with no trailing zero in
    /// `digits`: 2.54 is `(254, -2)`, 6300000 is `(63, 5)`.
    pub fn to_integer_and_exponent(self) -> (i128, i32) {
        let mantissa = self.mantissa.normalize();
        (mantissa.mantissa(), self.exponent - mantissa.scale() as i32)
    }

    /// `10^exp`, exactly.
    pub fn power_of_ten(exp: i32) -> Self {
        Self {
            mantissa: Decimal::ONE,
            exponent: exp,
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::ZERO
    }
}

impl core::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        let sign = |n: &Self| {
            if n.is_zero() {
                0
            } else if n.is_negative() {
                -1
            } else {
                1
            }
        };
        match sign(self).cmp(&sign(other)) {
            Ordering::Equal if sign(self) == 0 => Ordering::Equal,
            Ordering::Equal => {
                let magnitude = self
                    .exponent
                    .cmp(&other.exponent)
                    .then(self.mantissa.abs().cmp(&other.mantissa.abs()));
                if sign(self) < 0 {
                    magnitude.reverse()
                } else {
                    magnitude
                }
            }
            order => order,
        }
    }
}

impl fmt::Display for Number {
    /// Plain decimal notation for exponents from -7 to 20 ("0.0063", "6300000"),
    /// scientific notation otherwise ("1.380649e-23").
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mantissa.is_zero() {
            return f.write_str("0");
        }
        if !(-7..=20).contains(&self.exponent) {
            return write!(f, "{}e{}", self.mantissa, self.exponent);
        }
        if self.is_negative() {
            f.write_str("-")?;
        }
        let mut digits = Buffer::new();
        fmt::write(
            &mut digits,
            format_args!("{}", self.mantissa.abs().mantissa()),
        )?;
        let digits = digits.as_str();
        // value = 0.digits × 10^point
        let point = self.exponent + 1;
        if point <= 0 {
            f.write_str("0.")?;
            for _ in 0..-point {
                f.write_str("0")?;
            }
            f.write_str(digits)
        } else if point as usize >= digits.len() {
            f.write_str(digits)?;
            for _ in digits.len()..point as usize {
                f.write_str("0")?;
            }
            Ok(())
        } else {
            let (integer, fraction) = digits.split_at(point as usize);
            write!(f, "{integer}.{fraction}")
        }
    }
}

impl fmt::LowerExp for Number {
    /// Scientific notation: "2.54e0", "1e-7".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}e{}", self.mantissa, self.exponent)
    }
}

/// The number of decimal digits of a non-zero integer.
fn digit_count(mut n: u128) -> u32 {
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

/// A small stack buffer to format numbers without `std`.
struct Buffer {
    bytes: [u8; 64],
    len: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            bytes: [0; 64],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).expect("ASCII")
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        self.bytes
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    fn n(text: &str) -> Number {
        Number::parse(text).unwrap()
    }

    #[test]
    fn parses_decimals_exactly() {
        assert_eq!(n("2.54").to_string(), "2.54");
        assert_eq!(n("0.0063").to_string(), "0.0063");
        assert_eq!(n("6300000").to_string(), "6300000");
        assert_eq!(n("9.1093837139e-28").to_string(), "9.1093837139e-28");
        assert_eq!(n("-273.15").to_string(), "-273.15");
        assert_eq!(n("000.000").to_string(), "0");
        assert!(Number::parse("").is_none());
        assert!(Number::parse(".").is_none());
        assert!(Number::parse("1.2.3").is_none());
        assert!(Number::parse("1e").is_none());
    }

    #[test]
    fn rounds_digits_beyond_28() {
        let pi = n("3.1415926535897932384626433832795028841971693993751058209749445923");
        assert_eq!(pi.to_string(), "3.141592653589793238462643383");
        assert_eq!(
            n("123456789012345678901234567890123").to_string(),
            "1.234567890123456789012345679e32"
        );
    }

    #[test]
    fn keeps_any_magnitude() {
        let electron = n("9.1093837139e-28");
        let proton = n("1.67262192595e-24");
        let ratio = proton.checked_div(electron).unwrap();
        assert!((ratio.to_f64() - 1836.152673426).abs() < 1e-8);
        assert_eq!(
            Number::power_of_ten(24)
                .checked_powi(3)
                .unwrap()
                .to_string(),
            "1e72"
        );
        assert_eq!(n("1e-300").checked_mul(n("1e-300")).unwrap().to_f64(), 0.0);
    }

    #[test]
    fn arithmetic_is_decimal() {
        assert_eq!(n("6.3").checked_mul(n("0.001")).unwrap(), n("0.0063"));
        assert_eq!(n("0.1").checked_add(n("0.2")).unwrap(), n("0.3"));
        assert_eq!(n("373.15").checked_sub(n("273.15")).unwrap(), n("100"));
        assert_eq!(n("1").checked_add(n("1e-40")).unwrap(), n("1"));
        // 491.67 × 5/9 - 273.15 cancels to zero, not to the rounding error of 5/9
        let kelvin = n("491.67")
            .checked_mul(n("5").checked_div(n("9")).unwrap())
            .unwrap();
        assert_eq!(kelvin.checked_sub(n("273.15")).unwrap(), Number::ZERO);
        assert_eq!(n("2").checked_powi(-2).unwrap(), n("0.25"));
        assert!(n("1").checked_div(Number::ZERO).is_none());
        assert!(Number::ZERO.checked_powi(-1).is_none());
    }

    #[test]
    fn converts_from_and_to_f64() {
        assert_eq!(Number::from_f64(0.1).unwrap(), n("0.1"));
        assert_eq!(Number::from_f64(6.3e-30).unwrap(), n("6.3e-30"));
        assert_eq!(Number::from_f64(-0.0).unwrap(), Number::ZERO);
        assert!(Number::from_f64(f64::NAN).is_none());
        assert_eq!(n("0.0254").to_f64(), 0.0254);
        assert_eq!(n("1e400").to_f64(), f64::INFINITY);
    }

    #[test]
    fn orders_by_value() {
        assert!(n("-2") < n("-1"));
        assert!(n("-1") < Number::ZERO);
        assert!(n("0.5") < n("2"));
        assert!(n("9") < n("10"));
        assert_eq!(n("1.0").cmp(&n("1")), Ordering::Equal);
    }
}
