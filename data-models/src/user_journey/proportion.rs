use std::ops::{Deref, Mul};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProportionError {
    #[error("Invalid proportion `{0}`, must be between 0.0 and 1.0")]
    InvalidProportion(f64),
}

/// Represents a proportion of the total number of users between 0.0 and 1.0 inclusive.
///
/// To create a proportion both safely and accurately, you can use [`TryFrom`] on a float.
///
/// ```
/// use data_models::prelude::*;
///
/// assert!(Proportion::try_from(0.5).is_ok()); // Representing 50% of user
/// assert!(Proportion::try_from(-1.0).is_err()); // Would represent -100% which makes no sense
/// ```
///
/// You can also create a saturated proportion if you're ok with your proportion potentially not
/// being exactly representative of the number you started with
///
/// ```
/// use data_models::prelude::*;
///
/// assert_eq!(0.5.to_saturated_proportion(), 0.5); // Representing 50% of users
/// assert_eq!((-1.0).to_saturated_proportion(), 0.0); // Less than zero saturates to zero
/// assert_eq!(1.1.to_saturated_proportion(), 1.0); // More than one saturates to one
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Proportion(f64);

impl Proportion {
    /// Safe constructor to create a proportion of 0%
    /// ```
    /// use data_models::prelude::*;
    ///
    /// assert_eq!(Proportion::none(), 0.0);
    /// ```
    #[must_use]
    pub const fn none() -> Self {
        Self(0.0)
    }

    /// Safe constructor to create a proportion of 100%
    /// ```
    /// use data_models::prelude::*;
    ///
    /// assert_eq!(Proportion::all(), 1.0);
    /// ```
    #[must_use]
    pub const fn all() -> Self {
        Self(1.0)
    }

    /// Lets you split a proportion by another proportion. The first will be the proportion you
    /// ```
    /// use data_models::prelude::*;
    ///
    /// # let approximately_eq_f64 = |left: f64, right: f64| (left - right).abs() < 0.00001;
    /// #
    /// let p1 = 0.50.to_saturated_proportion();
    /// let (left, right) = p1.split_by(0.80.to_saturated_proportion());
    /// assert!(approximately_eq_f64(*left, 0.4));
    /// assert!(approximately_eq_f64(*right, 0.1));
    ///
    /// // The returned values are also `Proportion`s so can be split further.
    /// let (left2, right2) = left.split_by(0.25.to_saturated_proportion());
    /// assert!(approximately_eq_f64(*left, 0.4));
    /// assert!(approximately_eq_f64(*right, 0.1));
    /// ```
    #[must_use]
    pub fn split_by(self, proportion: Self) -> (Self, Self) {
        (
            Self(self.0 * proportion.0),
            Self(self.0 * (1.0 - proportion.0)),
        )
    }

    #[must_use]
    pub fn invert(self) -> Self {
        Self(1.0 - self.0)
    }

    #[must_use]
    pub fn split<M: Mul<Self> + Copy>(self, amount: M) -> (M::Output, M::Output) {
        (amount * self, amount * self.invert())
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

impl SaturatingProportion for f32 {
    fn to_saturated_proportion(self) -> Proportion {
        Proportion(f64::from(self).clamp(0.0, 1.0))
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

impl Mul<Proportion> for f64 {
    type Output = Self;

    fn mul(self, rhs: Proportion) -> Self::Output {
        self * *rhs
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
    fn test_split_by() {
        let p = 0.50.to_saturated_proportion();
        let (left, right) = p.split_by(0.80.to_saturated_proportion());
        assert!(approximately_eq_f64(*left, 0.4));
        assert!(approximately_eq_f64(*right, 0.1));
    }

    #[test]
    fn test_split() {
        let input = 100.0;
        let p = 0.80.to_saturated_proportion();
        let (left, right) = p.split(input);
        assert!(approximately_eq_f64(left, 80.0));
        assert!(approximately_eq_f64(right, 20.0));
    }
}
