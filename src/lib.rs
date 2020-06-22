//! Display human readable file sizes.
//!
//! # Limitation
//!
//! Displayed units go up to Exabyte (2^60).
//!
//! # Example
//!
//! ```
//! use size_display::Size;
//!
//! assert_eq!("24", format!("{}", Size(24)));
//! assert_eq!("4.2G", format!("{:.1}", Size(4509715660)));
//! ```

use std::fmt;

pub const KILO: u64 = 1024;
pub const MEGA: u64 = KILO * KILO;
pub const GIGA: u64 = MEGA * KILO;
pub const TERA: u64 = GIGA * KILO;
pub const PETA: u64 = TERA * KILO;
pub const EXA: u64 = PETA * KILO;

/// File size type with human-readable [`Display`].
///
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
pub struct Size(pub u64);

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Size {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.0;

        let (unit, char) = match bytes {
            e if e >= EXA => (EXA, "E"),
            p if p >= PETA => (PETA, "P"),
            t if t >= TERA => (TERA, "T"),
            g if g >= GIGA => (GIGA, "G"),
            m if m >= MEGA => (MEGA, "M"),
            k if k >= KILO => (KILO, "K"),
            _ => (1, ""),
        };

        match bytes % unit {
            0 => <u64 as fmt::Display>::fmt(&(bytes / unit), f),
            _ => <f64 as fmt::Display>::fmt(&(bytes as f64 / unit as f64), f),
        }?;

        if char != "" {
            write!(f, "{}", char)?;
        }

        Ok(())
    }
}
