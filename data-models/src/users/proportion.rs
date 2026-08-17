use std::ops::Deref;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProportionError {
    #[error("Invalid proportion `{0}`, must be between 0.0 and 1.0")]
    InvalidProportion(f64),
}

/// Represents a proportion of the total number of users between 0.0 and 1.0 inclusive
///
/// ```
/// use data_models::prelude::*;
///
/// assert!(Proportion::try_from(0.5).is_ok()); // Representing 50% of users
/// assert!(Proportion::try_from(-1.0).is_err()); // Would represent -100% which makes no sense
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Proportion(f64);

impl Proportion {
    #[must_use]
    pub const fn none() -> Self {
        Self(0.0)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(1.0)
    }

    #[must_use]
    pub fn split(self, proportion: Self) -> (Self, Self) {
        (
            Self(self.0 * proportion.0),
            Self(self.0 * (1.0 - proportion.0)),
        )
    }
}

pub trait SaturatingProportion {
    fn to_saturated_proportion(self) -> Proportion;
}

impl SaturatingProportion for f64 {
    fn to_saturated_proportion(self) -> Proportion {
        Proportion(self.clamp(0.0, 1.0))
    }
}

impl TryFrom<f64> for Proportion {
    type Error = ProportionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ProportionError::InvalidProportion(value))
        }
    }
}

impl Deref for Proportion {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<f64> for Proportion {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::approximately_eq_f64;

    #[test]
    fn test_none() {
        assert_eq!(Proportion::none(), 0.0);
    }

    #[test]
    fn test_all() {
        assert_eq!(Proportion::all(), 1.0);
    }

    #[test]
    fn test_split() {
        let p = 0.50.to_saturated_proportion();
        let (left, right) = p.split(0.80.to_saturated_proportion());
        assert!(approximately_eq_f64(*left, 0.4));
        assert!(approximately_eq_f64(*right, 0.1));
    }
}
