use core::fmt;
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

    /// Invert a proportion, so 25% will become 75%
    /// ```
    /// use data_models::prelude::*;
    ///
    /// let one_quarter = 0.25.to_saturated_proportion();
    /// let three_quarters = one_quarter.invert();
    /// assert_eq!(three_quarters, 0.75)
    /// ```
    #[must_use]
    pub fn invert(self) -> Self {
        Self(1.0 - self.0)
    }

    /// Split a number by the proportion
    /// ```
    /// use data_models::prelude::*;
    ///
    /// let input = 100.0;
    /// let p = 0.25.to_saturated_proportion();
    /// let (left, right) = p.split(input);
    /// assert_eq!(left, 25.0);
    /// assert_eq!(right, 75.0);
    /// ```
    #[must_use]
    pub fn split<M: Mul<Self> + Copy>(self, amount: M) -> (M::Output, M::Output) {
        (amount * self, amount * self.invert())
    }

    /// Allows you to add two proportions. Since a proportion can not be greater than `1.0`
    ///
    /// ```
    /// use data_models::prelude::*;
    ///
    /// let one_quarter = 0.25.to_saturated_proportion();
    /// let one_half = 0.5.to_saturated_proportion();
    ///
    /// // 0.25 + 0.5 = 0.75
    /// assert_eq!(one_quarter.saturating_add(one_half), 0.75.to_saturated_proportion());
    ///
    /// // 0.5 + 0.5 + 0.5 = 1.0 as can not be greater than 1.0
    /// assert_eq!(one_half.saturating_add(one_half).saturating_add(one_half), Proportion::all());
    /// ```
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        (*self + *other).to_saturated_proportion()
    }
}

pub trait SaturatingProportion {
    fn to_saturated_proportion(self) -> Proportion;
}


/// Turns an `f64` into a `Proportion`. If the `f64` was greater than `1.0` or less than `0.0` it
/// will be clamped.
/// ```
/// use data_models::prelude::*;
///
/// // Converts numbers between 0.0 and 1.0 ok
/// assert_eq!(*0.5_f64.to_saturated_proportion(), 0.5);
/// // Numbers greater than 1.0 cap out at 1.0
/// assert_eq!(10_f64.to_saturated_proportion(), 1.0);
/// // Numbers less than 0.0 cap out at 0.0
/// assert_eq!((-10_f64).to_saturated_proportion(), 0.0);
/// ```
impl SaturatingProportion for f64 {
    fn to_saturated_proportion(self) -> Proportion {
        Proportion(self.clamp(0.0, 1.0))
    }
}

/// Turns an `f32` into a `Proportion`. If the `f32` was greater than `1.0` or less than `0.0` it
/// will be clamped.
/// ```
/// use data_models::prelude::*;
///
/// // Converts numbers between 0.0 and 1.0 ok
/// assert_eq!(0.5_f32.to_saturated_proportion(), 0.5);
/// // Numbers greater than 1.0 cap out at 1.0
/// assert_eq!(10_f32.to_saturated_proportion(), 1.0);
/// // Numbers less than 0.0 cap out at 0.0
/// assert_eq!((-10_f32).to_saturated_proportion(), 0.0);
/// ```
impl SaturatingProportion for f32 {
    fn to_saturated_proportion(self) -> Proportion {
        Proportion(f64::from(self).clamp(0.0, 1.0))
    }
}

/// You can attempt to convert an `f64` to a proportion directly, however if the number is not
/// between `0.0` and `1.0` then you will get an error. This could be preferable to using
/// [`SaturatingProportion::to_saturated_proportion`]
/// ```
/// use data_models::prelude::*;
///
/// let good_proportion: Result<Proportion, _> = 0.9.try_into();
/// assert!(good_proportion.is_ok());
/// let bad_proportion: Result<Proportion, _> = 1.1.try_into();
/// assert!(bad_proportion.is_err());
/// ```
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

impl Mul for Proportion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl fmt::Display for Proportion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
        let p1 = 0.50.to_saturated_proportion();
        let (left, right) = p1.split_by(0.80.to_saturated_proportion());
        assert!(approximately_eq_f64(*left, 0.4));
        assert!(approximately_eq_f64(*right, 0.1));

        // The returned values are also `Proportion`s so can be split further.
        let (left2, right2) = left.split_by(0.25.to_saturated_proportion());
        assert!(approximately_eq_f64(*left2, 0.1), "{left2} != 0.1");
        assert!(approximately_eq_f64(*right2, 0.3), "{right2} != 0.3");
    }

    #[test]
    fn test_split() {
        let input = 100.0;
        let p = 0.80.to_saturated_proportion();
        let (left, right) = p.split(input);
        assert!(approximately_eq_f64(left, 80.0));
        assert!(approximately_eq_f64(right, 20.0));
    }

    #[test]
    fn test_to_saturated_proportion_f32() {
        // Converts numbers between 0.0 and 1.0 ok
        assert_eq!(0.5_f32.to_saturated_proportion(), 0.5);
        // Numbers greater than 1.0 cap out at 1.0
        assert_eq!(10_f32.to_saturated_proportion(), 1.0);
        // Numbers less than 0.0 cap out at 0.0
        assert_eq!((-10_f32).to_saturated_proportion(), 0.0);
    }

    #[test]
    fn test_try_from_f64() {
        let good_proportion: Result<Proportion, _> = 0.9.try_into();
        assert!(good_proportion.is_ok());
        let bad_proportion: Result<Proportion, _> = 1.1.try_into();
        assert!(bad_proportion.is_err());
    }

    #[test]
    fn test_display() {
        let good_proportion: Proportion = 0.9.try_into().unwrap();
        assert_eq!(format!("{good_proportion}"), "0.9");
    }
}
