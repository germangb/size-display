//! # size-display
//!
//! Display human readable file sizes.
//!
//! ## Limitation
//!
//! Displayed units go up to Exabyte (2^60).
//!
//! ## Example
//!
//! ```
//! use size_display::Size;
//!
//! assert_eq!("24", format!("{}", Size(24)));
//! assert_eq!("4.2G", format!("{:.1}", Size(4509715660)));
//! ```

use std::fmt;
use std::fmt::Debug;

pub const K: u64 = 1024;
pub const M: u64 = K * K;
pub const G: u64 = M * K;
pub const T: u64 = G * K;
pub const P: u64 = T * K;
pub const E: u64 = P * K;

/// File size type with human-readable `Display`.
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
            e if e >= E => (E, "E"),
            p if p >= P => (P, "P"),
            t if t >= T => (T, "T"),
            g if g >= G => (G, "G"),
            m if m >= M => (M, "M"),
            k if k >= K => (K, "K"),
            b => (1, ""),
        };

        format_size(f, self.0, unit)?;
        write!(f, "{}", char)?;
        Ok(())
    }
}

fn format_size(f: &mut fmt::Formatter, size: u64, unit: u64) -> fmt::Result {
    match size % unit {
        0 => <u64 as fmt::Display>::fmt(&(size / unit), f),
        _ => <f64 as fmt::Display>::fmt(&(size as f64 / unit as f64), f),
    }
}
