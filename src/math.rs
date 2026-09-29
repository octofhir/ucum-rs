//! `f64` math that works with and without `std`.
//!
//! `core` does not provide transcendental or rounding functions on `f64`.
//! With the `std` feature these delegate to the inherent `f64` methods;
//! without it they are backed by `libm`.

macro_rules! float_fn {
    ($(#[$doc:meta])* $name:ident($($arg:ident),*) => std: $std:expr, libm: $libm:expr) => {
        $(#[$doc])*
        #[inline]
        pub(crate) fn $name($($arg: f64),*) -> f64 {
            #[cfg(feature = "std")]
            {
                $std
            }
            #[cfg(not(feature = "std"))]
            {
                $libm
            }
        }
    };
}

float_fn!(powf(x, n) => std: x.powf(n), libm: libm::pow(x, n));
float_fn!(exp(x) => std: x.exp(), libm: libm::exp(x));
float_fn!(ln(x) => std: x.ln(), libm: libm::log(x));
float_fn!(log(x, base) => std: x.log(base), libm: libm::log(x) / libm::log(base));
float_fn!(log10(x) => std: x.log10(), libm: libm::log10(x));
float_fn!(log2(x) => std: x.log2(), libm: libm::log2(x));
float_fn!(tan(x) => std: x.tan(), libm: libm::tan(x));
float_fn!(floor(x) => std: x.floor(), libm: libm::floor(x));
float_fn!(round(x) => std: x.round(), libm: libm::round(x));
float_fn!(fract(x) => std: x.fract(), libm: x - libm::trunc(x));

/// Raise `x` to an integer power.
#[inline]
pub(crate) fn powi(x: f64, n: i32) -> f64 {
    #[cfg(feature = "std")]
    {
        x.powi(n)
    }
    #[cfg(not(feature = "std"))]
    {
        libm::pow(x, n as f64)
    }
}
