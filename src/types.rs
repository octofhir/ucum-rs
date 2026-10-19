use crate::number::Number;
use core::fmt;

/// SI prefix such as `k` (kilo) or `m` (milli).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Prefix {
    /// Case sensitive symbol (`da`, `Ki`)
    pub symbol: &'static str,
    /// Case insensitive symbol (`DA`, `KIB`, UCUM §3.4)
    pub symbol_ci: &'static str,
    /// Value of the prefix (1000 for `k`, 1024 for `Ki`)
    pub factor: Number,
    /// Name (`kilo`)
    pub display_name: &'static str,
    /// Print symbol (`k`, `μ`); display only, not normative
    pub print_symbol: &'static str,
}

/// Exponents of the seven UCUM base units (UCUM §28): meter, second, gram, radian,
/// kelvin, coulomb and candela, in this order.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dimension(pub [i8; 7]);

impl Dimension {
    /// Codes of the base units, in the order of the exponents.
    pub const BASE_UNITS: [&'static str; 7] = ["m", "s", "g", "rad", "K", "C", "cd"];

    /// Dimension symbols of the base units (length, time, mass, plane angle,
    /// temperature, electric charge, luminous intensity).
    pub const SYMBOLS: [&'static str; 7] = ["L", "T", "M", "A", "Θ", "Q", "F"];

    /// The dimension of the unity.
    pub const fn zero() -> Self {
        Self([0; 7])
    }

    /// Whether every exponent is zero.
    pub const fn is_dimensionless(&self) -> bool {
        matches!(self.0, [0, 0, 0, 0, 0, 0, 0])
    }
}

impl fmt::Display for Dimension {
    /// "L.T-2", or "1" for the unity.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return f.write_str("1");
        }
        let mut first = true;
        for (symbol, &exp) in Self::SYMBOLS.iter().zip(&self.0) {
            if exp == 0 {
                continue;
            }
            if !first {
                f.write_str(".")?;
            }
            first = false;
            f.write_str(symbol)?;
            if exp != 1 {
                write!(f, "{exp}")?;
            }
        }
        Ok(())
    }
}

/// Numerical quantity paired with a unit expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity {
    pub value: f64,
    pub unit: crate::ast::UnitExpr,
}

/// How a unit relates to the proper units of the base.
///
/// - `None`: a proper unit, `factor` times the base units of `dim` (UCUM §20).
/// - `Arbitrary`: an arbitrary unit (UCUM §24-26), commensurable with no other unit. Its
///   `arbitrary` atoms take part in commensurability like extra base units: `[IU]/mL` is
///   commensurable with `[IU]/L` but not with `[arb'U]/mL` or `/mL`.
/// - Any other kind: a special unit (UCUM §21-23) on a non-ratio scale, defined by a
///   function `f` and its inverse, and by its corresponding proper unit `u`, whose factor
///   and dimension are those of the record. A measurement `r` in the special unit is the
///   proper quantity `f⁻¹(r) × u`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecialKind {
    /// A proper unit.
    None,
    /// An arbitrary unit (`[IU]`, `[arb'U]`).
    Arbitrary,
    /// Degree Celsius, `cel(1 K)`: `f(x) = x - 273.15`.
    Celsius,
    /// Degree Fahrenheit, `degf(5 K/9)`: `f(x) = x - 459.67`.
    Fahrenheit,
    /// Degree Réaumur, `degre(5 K/4)`: `f(x) = x - 218.52`.
    Reaumur,
    /// Bel and the levels of power (`B`, `B[W]`), `lg`: `f(x) = lg x`.
    Lg,
    /// Levels of field quantities (`B[V]`, `B[SPL]`), `2lg`: `f(x) = 2 lg x`.
    LgTimes2,
    /// Neper, `ln`: `f(x) = ln x`.
    Ln,
    /// Bit, `ld`: `f(x) = ld x`.
    Ld,
    /// pH, `pH(1 mol/l)`: `f(x) = -lg x`.
    PH,
    /// Homeopathic potency of the decimal series, `hpX`: `f(x) = -lg x`.
    HpX,
    /// Homeopathic potency of the centesimal series, `hpC`: `f(x) = -lg x / 2`.
    HpC,
    /// Homeopathic potency of the millesimal series, `hpM`: `f(x) = -lg x / 3`.
    HpM,
    /// Homeopathic potency of the quintamillesimal series, `hpQ`:
    /// `f(x) = -ln x / ln 50000`.
    HpQ,
    /// Prism diopter and percent of slope, `100tan(1 rad)`: `f(x) = 100 tan x`.
    TanTimes100,
    /// Amplitude spectral density, `sqrt(1 m2/s4/Hz)`: `f(x) = √x`.
    Sqrt,
}

impl SpecialKind {
    /// Whether this is a special unit (UCUM §21), defined by a function.
    pub fn is_special(&self) -> bool {
        !matches!(self, SpecialKind::None | SpecialKind::Arbitrary)
    }

    /// Whether this is an arbitrary unit (UCUM §24).
    pub fn is_arbitrary(&self) -> bool {
        matches!(self, SpecialKind::Arbitrary)
    }

    /// For a special unit on an interval scale (the temperatures), the offset `o` of
    /// `f(x) = x - o`.
    pub(crate) fn offset(&self) -> Option<Number> {
        match self {
            SpecialKind::Celsius => Number::parse("273.15"),
            SpecialKind::Fahrenheit => Number::parse("459.67"),
            SpecialKind::Reaumur => Number::parse("218.52"),
            _ => None,
        }
    }

    /// For a logarithmic special unit, `(k, b)` such that `f(x) = k · log_b(x)`, with `k` a
    /// fraction `(numerator, denominator)`.
    pub(crate) fn logarithm(&self) -> Option<((i64, i64), f64)> {
        const E: f64 = core::f64::consts::E;
        match self {
            SpecialKind::Lg => Some(((1, 1), 10.0)),
            SpecialKind::LgTimes2 => Some(((2, 1), 10.0)),
            SpecialKind::Ln => Some(((1, 1), E)),
            SpecialKind::Ld => Some(((1, 1), 2.0)),
            SpecialKind::PH | SpecialKind::HpX => Some(((-1, 1), 10.0)),
            SpecialKind::HpC => Some(((-1, 2), 10.0)),
            SpecialKind::HpM => Some(((-1, 3), 10.0)),
            SpecialKind::HpQ => Some(((-1, 1), 50000.0)),
            _ => None,
        }
    }

    /// `f(x)`: the value in the special unit of a proper quantity `x` times its
    /// corresponding unit. `None` outside the domain of `f`: `f` and `f⁻¹` are mutually
    /// inverse real functions (UCUM §21.2), so a logarithm takes `x > 0`, a square root
    /// `x ≥ 0` and `100 tan x` the principal branch, `|x| < π/2`.
    pub(crate) fn forward(&self, x: f64) -> Option<f64> {
        if let Some(offset) = self.offset() {
            return Some(x - offset.to_f64());
        }
        if let Some(((num, den), base)) = self.logarithm() {
            return (x > 0.0)
                .then(|| num as f64 * crate::math::ln(x) / (den as f64 * crate::math::ln(base)));
        }
        match self {
            // `FRAC_PI_2` is below π/2, so every `f64` up to it is inside the open interval
            SpecialKind::TanTimes100 => {
                (x.abs() <= core::f64::consts::FRAC_PI_2).then(|| 100.0 * crate::math::tan(x))
            }
            SpecialKind::Sqrt => (x >= 0.0).then(|| crate::math::sqrt(x)),
            _ => Some(x),
        }
    }

    /// `f⁻¹(r)`: the proper quantity, in multiples of the corresponding unit, of a value
    /// `r` in the special unit. `None` outside the domain of `f⁻¹`, the range of `f`: a
    /// square root is never negative.
    pub(crate) fn inverse(&self, r: f64) -> Option<f64> {
        if let Some(offset) = self.offset() {
            return Some(r + offset.to_f64());
        }
        if let Some(((num, den), base)) = self.logarithm() {
            return Some(crate::math::powf(base, r * den as f64 / num as f64));
        }
        match self {
            SpecialKind::TanTimes100 => Some(crate::math::atan(r / 100.0)),
            SpecialKind::Sqrt => (r >= 0.0).then_some(r * r),
            _ => Some(r),
        }
    }
}

/// A unit atom of the UCUM tables.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct UnitRecord {
    /// Case sensitive code (`Pa`, `[in_i]`)
    pub code: &'static str,
    /// Case insensitive code (`PAL`, `[IN_I]`, UCUM §3.4)
    pub code_ci: &'static str,
    /// Dimension in base units; for a special unit, of its corresponding proper unit
    pub dim: Dimension,
    /// Magnitude in base units; for a special unit, of its corresponding proper unit
    pub factor: Number,
    /// Proper, arbitrary or special unit
    pub special: SpecialKind,
    /// Arbitrary atoms the unit reduces to, with their exponents (`[IU]` is `[iU]`)
    pub arbitrary: &'static [(&'static str, i32)],
    /// Whether this is one of the seven base units
    pub is_base: bool,
    /// Whether the unit takes prefixes (UCUM §11)
    pub is_metric: bool,
    /// Class in the UCUM tables (`si`, `clinical`, `levels`); empty for base units
    pub class: &'static str,
    /// Kind of quantity (`length`, `pressure`)
    pub property: &'static str,
    /// Name (`pascal`)
    pub display_name: &'static str,
    /// Print symbol (`Pa`); display only, not normative
    pub print_symbol: &'static str,
}

// Here rather than in `number`: the build script includes `number` and has no serde
#[cfg(feature = "serde")]
impl serde::Serialize for Number {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Number {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <alloc::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        Number::parse(&text).ok_or_else(|| serde::de::Error::custom("invalid number"))
    }
}
