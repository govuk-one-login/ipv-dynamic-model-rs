use crate::prelude::{Proportion, RequestsPerSecond, UserRequirement};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Users {
    requests_per_second: RequestsPerSecond,
    requirements: HashMap<UserRequirement, Proportion>,
    cis: Vec<String>,
    mitigated_cis: Vec<String>,
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
            mitigated_cis: Vec::new(),
        }
    }

    #[must_use]
    pub const fn get_rps(&self) -> RequestsPerSecond {
        self.requests_per_second
    }

    #[must_use]
    pub fn get_requirement(&self, requirement: &UserRequirement) -> Proportion {
        self.requirements
            .get(requirement)
            .copied()
            .unwrap_or_default()
    }

    /// Set a proportion of users who can match a particular requirement
    pub fn set_requirement(&mut self, requirement: &UserRequirement, proportion: Proportion) {
        self.requirements.insert(*requirement, proportion);
    }

    /// Allows us to determine if there are too many contraindicators, even if they have been
    /// mitigated
    #[must_use]
    pub const fn total_ci_count(&self) -> usize {
        self.cis.len()
    }

    /// Get a list of any unmitigated CIs
    #[must_use]
    pub fn get_unmitigated_cis(&self) -> Vec<&String> {
        self.cis
            .iter()
            .filter(|ci| self.mitigated_cis.contains(ci))
            .collect()
    }

    /// Split the user pool by a specific requirement without changing any of the other proportions
    #[must_use]
    pub fn split_by(self, requirement: &UserRequirement) -> (Self, Self) {
        let proportion = self.get_requirement(requirement);
        let (left_rps, right_rps) = proportion.split(self.get_rps());

        let mut left = self.clone();
        let mut right = self;

        left.requests_per_second = left_rps;
        left.set_requirement(requirement, Proportion::all());

        right.requests_per_second = right_rps;
        left.set_requirement(requirement, Proportion::none());

        (left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::SaturatingProportion;

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
}
