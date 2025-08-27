//! Error types for Pell equation solving

use std::fmt;
use crate::utils::isqrt_u64;

/// Errors that can occur when solving Pell equations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PellError {
    /// D must be greater than 1
    InvalidD(u64),
    /// D must not be a perfect square
    PerfectSquare(u64),
    /// k must be greater than 0
    InvalidK(u64),
}

impl fmt::Display for PellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PellError::InvalidD(d) => write!(f, "D must be > 1, got {d}"),
            PellError::PerfectSquare(d) => write!(f, "D must be non-square, got {d} which is {}²", isqrt_u64(*d)),
            PellError::InvalidK(k) => write!(f, "k must be > 0, got {k}"),
        }
    }
}

impl std::error::Error for PellError {}