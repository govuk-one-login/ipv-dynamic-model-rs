use serde::{Deserialize, Serialize};
use std::ops::Deref;
use thiserror::Error;

#[derive(Copy, Clone, Debug, Error)]
pub enum RequestsPerSecondError {
    #[error("Requests Per Second must be greater than zero, given (aprox) {0:.2}")]
    NegativeNumber(f64),
}

#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug, PartialEq)]
pub struct RequestsPerSecond(f64);

impl RequestsPerSecond {
    /// Try to make a new [`RequestsPerSecond`] object, so long as its valid:
    ///
    /// ```rust
    /// use data_models::prelude::*;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let requests_per_second = RequestsPerSecond::new(50.0)?;
    /// assert_eq!(requests_per_second.as_f64(), 50.0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the number is less than zero
    ///
    /// ```rust
    /// use data_models::prelude::*;
    ///
    /// assert!(RequestsPerSecond::new(-50.0).is_err());
    /// ```
    pub fn new(value: f64) -> Result<Self, RequestsPerSecondError> {
        let test_value = Self(value);
        test_value.is_valid()
    }

    /// Check against rules for requests per second
    ///
    /// # Errors
    ///
    /// Will return false if the number of requests per second is somehow negative
    fn is_valid(self) -> Result<Self, RequestsPerSecondError> {
        if self.0 < 0.0 {
            return Err(RequestsPerSecondError::NegativeNumber(self.0));
        }
        Ok(self)
    }

    /// Reveal the raw data as an `f64`
    ///
    /// ```rust
    /// use data_models::prelude::*;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let requests_per_second = RequestsPerSecond::new(50.0)?;
    /// assert_eq!(requests_per_second.as_f64(), 50.0);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub const fn as_f64(&self) -> f64 {
        self.0
    }

    /// Subtract a number of [`RequestsPerSecond`] and return the remaining capacity. This number
    /// will always be non-negative, so if the subtracting RPS is greater than this RPS, you will
    /// get an RPS of 0.0
    ///
    /// ```rust
    /// use data_models::prelude::*;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let requests_per_second = RequestsPerSecond::new(50.0)?;
    /// let moderate_usage = RequestsPerSecond::new(25.0)?;
    /// assert_eq!(requests_per_second.remaining_capacity(moderate_usage).as_f64(), 25.0);
    ///
    ///
    /// let extreme_usage = RequestsPerSecond::new(100.0)?;
    /// assert_eq!(requests_per_second.remaining_capacity(extreme_usage).as_f64(), 0.0);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn remaining_capacity(&self, subtracting: Self) -> Self {
        Self(f64::max(self.0 - subtracting.0, 0.0))
    }
}

impl Deref for RequestsPerSecond {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test_utils {
    use crate::prelude::RequestsPerSecond;
    use crate::test_utils::CreateTestSubject;
    use rand::random_range;

    impl CreateTestSubject for RequestsPerSecond {
        fn create_test_subject() -> Self {
            Self(random_range(0.0..200.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rps = RequestsPerSecond::new(50.0).unwrap();
        assert_eq!(*rps, 50.0);

        assert!(RequestsPerSecond::new(-50.0).is_err());
    }

    #[test]
    fn test_as_f64() {
        let rps = RequestsPerSecond::new(50.0).unwrap();
        assert_eq!(rps.as_f64(), 50.0);
    }

    #[test]
    fn test_remaining_capacity() {
        let rps_05 = RequestsPerSecond::new(5.0).unwrap();
        let rps_10 = RequestsPerSecond::new(10.0).unwrap();

        assert_eq!(*rps_10.remaining_capacity(rps_05), 5.0);
        assert_eq!(*rps_05.remaining_capacity(rps_10), 0.0); // Never less than 0
    }

    #[test]
    fn test_deref() {
        let rps = RequestsPerSecond::new(50.0).unwrap();
        assert_eq!(*rps, 50.0);
    }
}
