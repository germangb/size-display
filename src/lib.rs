//! # `size-display`
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
//! assert_eq!("4.2G", format!("{}", Size(4509715660)));
//! ```

use std::fmt;

pub const K: u64 = 1024;
pub const M: u64 = K * K;
pub const G: u64 = K * K * K;
pub const T: u64 = K * K * K * K;
pub const P: u64 = K * K * K * K * K;
pub const E: u64 = K * K * K * K * K * K;

/// File size type with human-readable `Debug`.
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

        if bytes >= E {
            format_size(f, self.0, E)?;
            write!(f, "E")?;
        } else if bytes >= P {
            format_size(f, self.0, P)?;
            write!(f, "P")?;
        } else if bytes >= T {
            format_size(f, self.0, T)?;
            write!(f, "T")?;
        } else if bytes >= G {
            format_size(f, self.0, G)?;
            write!(f, "G")?;
        } else if bytes >= M {
            format_size(f, self.0, M)?;
            write!(f, "M")?;
        } else if bytes >= K {
            format_size(f, self.0, K)?;
            write!(f, "K")?;
        } else {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

fn format_size(f: &mut fmt::Formatter, size: u64, unit: u64) -> fmt::Result {
    match size % unit {
        0 => write!(f, "{}", size / unit),
        _ => write!(f, "{:.1}", size as f32 / unit as f32),
    }
}
