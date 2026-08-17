use crate::prelude::*;

pub enum JourneyStep {
    Success(Users, Service),
    CouldNotUse(Users, Service),
    Failure(Users, Service),
}

pub enum CompletedStatus {
    Success(Users, IdentityProfile),
    Failure(Users),
}

/// A `Journey` represents a single path that a user is taking through the system
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Journey(Vec<Service>);

impl Journey {
    /// Is the individual service complete
    #[must_use]
    pub fn is_complete(&self) -> Option<CompletedStatus> {
        todo!()
    }

    pub fn possible_next_steps(&self) {
        todo!()
    }
}

/// A `Journeys` represents all Journeys a group of users could take through the system
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Journeys(Vec<Journey>);

impl Journeys {
    #[must_use]
    pub fn step(_all_services: &[Service]) -> bool {
        todo!()
    }
}
