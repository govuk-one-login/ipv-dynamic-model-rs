use crate::prelude::{Proportion, RequestsPerSecond, SaturatingProportion, UserRequirement};
use crate::user_journey::ci::{Ci, CiState};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Users {
    requests_per_second: RequestsPerSecond,
    requirements: HashMap<UserRequirement, Proportion>,
    cis: Vec<Ci>,
}

impl Users {
    #[must_use]
    pub const fn new(
        requests_per_second: RequestsPerSecond,
        requirements: HashMap<UserRequirement, Proportion>,
    ) -> Self {
        Self {
            requests_per_second,
            requirements,
            cis: Vec::new(),
        }
    }

    #[must_use]
    pub const fn get_rps(&self) -> RequestsPerSecond {
        self.requests_per_second
    }

    #[must_use]
    pub fn get_requirement(&self, requirement: UserRequirement) -> Proportion {
        self.requirements
            .get(&requirement)
            .copied()
            .unwrap_or_default()
    }

    /// Set a proportion of users who can match a particular requirement
    pub fn set_requirement(&mut self, requirement: UserRequirement, proportion: Proportion) {
        self.requirements.insert(requirement, proportion);
    }

    /// Allows us to determine if there are too many contraindicators, even if they have been
    /// mitigated
    #[must_use]
    pub const fn total_ci_count(&self) -> usize {
        self.cis.len()
    }

    /// Get a list of any unmitigated CIs
    #[must_use]
    pub fn get_unmitigated_cis(&self) -> Vec<&Ci> {
        self.cis
            .iter()
            .filter(|ci| ci.state() == CiState::Unmitigated)
            .collect()
    }

    #[must_use]
    pub fn take_proportion(&mut self, proportion: Proportion) -> Self {
        let (taken, remaining) = proportion.split(self.get_rps());
        let mut returned_users = self.clone();
        returned_users.requests_per_second = taken;
        self.requests_per_second = remaining;

        returned_users
    }

    #[must_use]
    pub fn take_count(&mut self, count: RequestsPerSecond) -> Self {
        let mut returned_users = self.clone();
        // Safe because neither possible number is 0
        returned_users.requests_per_second = f64::min(*self.requests_per_second, *count)
            .try_into()
            .unwrap();
        // Safe because can not be less than 0
        self.requests_per_second = f64::max(*self.requests_per_second - *count, 0.0)
            .try_into()
            .unwrap();

        returned_users
    }

    /// Split the user pool by a specific requirement without changing any of the other proportions
    #[must_use]
    pub fn split_by(&self, requirement: UserRequirement) -> (Self, Self) {
        let proportion = self.get_requirement(requirement);
        let (left_rps, right_rps) = proportion.split(self.get_rps());

        let mut left = self.clone();
        let mut right = self.clone();

        left.requests_per_second = left_rps;
        left.set_requirement(requirement, Proportion::all());

        right.requests_per_second = right_rps;
        right.set_requirement(requirement, Proportion::none());

        (left, right)
    }

    /// Only used for checking if two [`Users`] objects _can_ be combined, see the `Add`
    /// implementation
    fn has_unmitigated_cis_mitigated_by_other(&self, other: &Self) -> bool {
        let unmitigated_self: Vec<_> = self
            .cis
            .iter()
            .filter_map(|ci| match ci.state() {
                CiState::Unmitigated => Some(ci.label()),
                CiState::Mitigated => None,
            })
            .collect();
        other
            .cis
            .iter()
            .filter_map(|ci| match ci.state() {
                CiState::Unmitigated => None,
                CiState::Mitigated => Some(ci.label()),
            })
            .any(|label| unmitigated_self.contains(&label))
    }
}

#[derive(Debug, Error)]
pub enum UsersAddError {
    #[error("One side had unmitigated CIs the other side had mitigated")]
    CiMismatch,
}

impl Add for Users {
    type Output = Result<Users, UsersAddError>;

    fn add(self, rhs: Self) -> Self::Output {
        // We need to check if either side has mitigated CIs that the other side hasn't in which
        // case they can not be merged
        if self.has_unmitigated_cis_mitigated_by_other(&rhs)
            || rhs.has_unmitigated_cis_mitigated_by_other(&self)
        {
            return Err(UsersAddError::CiMismatch);
        }

        // Now we can combine the two lists
        let cis: Vec<_> = self
            .cis
            .into_iter()
            .chain(rhs.cis.into_iter())
            .unique()
            .collect();

        let total_rps = self.requests_per_second + rhs.requests_per_second;

        // We need to adjust the proportional amount to the new count for each of the other Users
        let self_proportion = (self.requests_per_second / total_rps).to_saturated_proportion();
        let rhs_proportion = (rhs.requests_per_second / total_rps).to_saturated_proportion();

        // Resize the proportions for self
        let mut requirements: HashMap<_, _> = self
            .requirements
            .into_iter()
            .map(|(requirement, proportion)| (requirement, proportion * self_proportion))
            .collect();

        // Now add in the rhs proportions
        rhs.requirements
            .into_iter()
            .map(|(requirement, proportion)| (requirement, proportion * rhs_proportion))
            .for_each(|(requirement, proportion)| {
                requirements
                    .entry(requirement)
                    .and_modify(|existing_proportion| {
                        *existing_proportion = existing_proportion.saturating_add(proportion)
                    })
                    .or_insert(proportion);
            });

        Ok(Users {
            requests_per_second: total_rps,
            requirements,
            cis,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::SaturatingProportion;
    use crate::test_utils::approximately_eq_f64;

    #[test]
    fn test_get_rps() {
        let users = Users::new(
            RequestsPerSecond::new(25.0).unwrap(),
            HashMap::from([(
                UserRequirement::DrivingLicense,
                0.80.to_saturated_proportion(),
            )]),
        );
        assert_eq!(*users.get_rps(), 25.0);
    }

    #[test]
    fn test_take_proportion() {
        let mut users = Users::new(
            RequestsPerSecond::new(25.0).unwrap(),
            HashMap::from([(
                UserRequirement::DrivingLicense,
                0.80.to_saturated_proportion(),
            )]),
        );
        let taken = users.take_proportion(0.2.try_into().unwrap());
        assert_eq!(*users.get_rps(), 20.0);
        assert_eq!(*taken.get_rps(), 5.0);
    }

    #[test]
    fn test_take_count() {
        let mut users = Users::new(
            RequestsPerSecond::new(25.0).unwrap(),
            HashMap::from([(
                UserRequirement::DrivingLicense,
                0.80.to_saturated_proportion(),
            )]),
        );

        // Take 5 of the 25
        let taken1 = users.take_count(5.0.try_into().unwrap());
        assert_eq!(*users.get_rps(), 20.0);
        assert_eq!(*taken1.get_rps(), 5.0);

        // Take more than remain
        let taken2 = users.take_count(100.0.try_into().unwrap());
        assert_eq!(*users.get_rps(), 0.0);
        assert_eq!(*taken2.get_rps(), 20.0);
    }

    #[test]
    fn test_split_by() {
        let mut users = Users::new(
            RequestsPerSecond::new(25.0).unwrap(),
            HashMap::from([
                (UserRequirement::SmartPhone, 0.80.to_saturated_proportion()),
                (UserRequirement::Passport, 0.50.to_saturated_proportion()),
            ]),
        );

        let (left, right) = users.split_by(UserRequirement::SmartPhone);
        assert!(approximately_eq_f64(*left.get_rps(), 20.0));
        assert!(approximately_eq_f64(*right.get_rps(), 5.0));
        assert_eq!(left.get_requirement(UserRequirement::SmartPhone), Proportion::all());
        assert_eq!(left.get_requirement(UserRequirement::Passport), 0.5.to_saturated_proportion());
        assert_eq!(right.get_requirement(UserRequirement::SmartPhone), Proportion::none());
        assert_eq!(right.get_requirement(UserRequirement::Passport), 0.5.to_saturated_proportion());
    }

    #[test]
    fn test_add() {
        let mut users = Users::new(
            RequestsPerSecond::new(25.0).unwrap(),
            HashMap::from([
                (UserRequirement::SmartPhone, 0.80.to_saturated_proportion()),
                (UserRequirement::Passport, 0.50.to_saturated_proportion()),
            ]),
        );

        let (left, right) = users.split_by(UserRequirement::SmartPhone);
        let added = (left + right).unwrap();

        assert!(approximately_eq_f64(*added.requests_per_second, 25.0));
        assert!(approximately_eq_f64(*added.requests_per_second, 25.0));
        assert!(approximately_eq_f64(*added.get_requirement(UserRequirement::SmartPhone), 0.8));
        assert!(approximately_eq_f64(*added.get_requirement(UserRequirement::Passport), 0.5));
    }
}
