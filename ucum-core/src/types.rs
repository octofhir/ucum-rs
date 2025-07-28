use core::fmt;

/// SI prefix such as `k` (kilo) or `m` (milli).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefix {
    pub symbol: &'static str,
    pub factor: f64,
    pub exponent: i8,
    pub display_name: &'static str,
}

/// Dimensional vector (M, L, T, I, Θ, N, J) per UCUM spec.
/// Optimized with repr(C) for consistent memory layout.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dimension(pub [i8; 7]);

impl Dimension {
    /// Create a zero dimension (dimensionless).
    pub const fn zero() -> Self {
        Self([0; 7])
    }
    
    /// Check if this dimension is dimensionless.
    pub const fn is_dimensionless(&self) -> bool {
        matches!(self.0, [0, 0, 0, 0, 0, 0, 0])
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.0;
        write!(
            f,
            "[M{} L{} T{} I{} Θ{} N{} J{}]",
            v[0], v[1], v[2], v[3], v[4], v[5], v[6]
        )
    }
}

/// Base unit record (e.g. metre, second, kelvin).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseUnit {
    pub code: &'static str,
    pub dim: Dimension,
    pub factor: f64,
    pub offset: f64,
}

/// Derived or custom unit that resolves to a base vector plus factor/offset.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DerivedUnit {
    pub code: &'static str,
    pub dim: Dimension,
    pub factor: f64,
    pub offset: f64,
}

/// Numerical quantity paired with a unit expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity {
    pub value: f64,
    pub unit: crate::ast::UnitExpr,
}

/// Record used in generated registry for both base and derived units.
/// Kind of special conversion required for a UCUM unit.
///
/// This enum represents the different kinds of special conversions that may be required
/// for UCUM units. Most units use simple linear scaling (the `None` variant), but some
/// units require more complex conversions.
///
/// ## Arbitrary Units
///
/// Arbitrary units (the `Arbitrary` variant) are a special case in UCUM. They are used
/// for units that are not defined in terms of any other unit, such as international units
/// (IU) or arbitrary units (arb'U). According to the UCUM specification (§24-26):
///
/// - Arbitrary units are enclosed in square brackets, e.g., `[IU]`, `[arb'U]`
/// - They are dimensionless with a factor of 1.0
/// - They are not commensurable with any other unit, including other arbitrary units
/// - They can be combined with other units (e.g., `[IU]/mL`)
/// - They can be prefixed (e.g., `k[IU]`)
///
/// In this implementation, arbitrary units are treated as dimensionless with a factor of 1.0,
/// but they are marked with the `Arbitrary` variant to ensure they are not commensurable
/// with other units.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecialKind {
    /// Regular linear scaling (the default).
    None,
    /// Linear offset (e.g., °C, °F).
    LinearOffset,
    /// Base-10 logarithmic unit (e.g., Bel, decibel).
    Log10,
    /// Natural logarithmic unit (Neper).
    Ln,
    /// TAN(x)*100 scaling (prism diopter).
    TanTimes100,
    /// Arbitrary unit (e.g., [IU], [arb'U]).
    /// These units are not commensurable with any other unit.
    Arbitrary,
}

impl SpecialKind {
    /// Returns the conversion ratio for this special unit type.
    /// For logarithmic units, this returns the base of the logarithm.
    pub fn ratio(&self) -> f64 {
        match self {
            SpecialKind::Log10 => 10.0,             // 10^(x) for B, 10^(x/10) for dB
            SpecialKind::Ln => std::f64::consts::E, // e^(x)
            _ => 1.0, // For None, LinearOffset, TanTimes100, Arbitrary
        }
    }

    /// Returns true if this is an arbitrary unit.
    /// Arbitrary units are not commensurable with any other unit.
    pub fn is_arbitrary(&self) -> bool {
        matches!(self, SpecialKind::Arbitrary)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitRecord {
    pub code: &'static str,
    pub dim: Dimension,
    pub factor: f64,
    pub offset: f64,
    pub special: SpecialKind,
    pub property: &'static str,
    pub display_name: &'static str,
}
