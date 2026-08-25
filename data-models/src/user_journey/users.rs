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

    pub fn set_requirement(&mut self, _requirement: &UserRequirement, _proportion: Proportion) {
        todo!()
        // self.requirements.entry(requirement)
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
}
