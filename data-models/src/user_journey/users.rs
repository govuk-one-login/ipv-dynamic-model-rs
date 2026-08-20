use crate::prelude::{Proportion, RequestsPerSecond, UserRequirement};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Users {
    requests_per_second: RequestsPerSecond,
    requirements: HashMap<UserRequirement, Proportion>,
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
}
