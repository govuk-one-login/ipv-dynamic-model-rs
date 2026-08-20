use crate::identity::Confidence;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum JourneyStep {
    Success(Users, Service),
    CouldNotUse(Users, Service),
    Failure(Users, Service),
}

impl JourneyStep {
    #[must_use]
    pub const fn get_service(&self) -> &Service {
        match self {
            Self::Success(_, service)
            | Self::CouldNotUse(_, service)
            | Self::Failure(_, service) => service,
        }
    }
}

pub enum CompletedStatus {
    Success(Users, IdentityProfile),
    Failure(Users),
}

/// A `Journey` represents a single path that a user is taking through the system
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Journey(Vec<JourneyStep>);

impl Journey {
    /// Is the individual service complete
    #[must_use]
    pub fn completed_status(&self) -> Option<CompletedStatus> {
        if let JourneyStep::Failure(users, _) = self.0.last()? {
            return Some(CompletedStatus::Failure(users.clone()));
        }
        todo!()
    }

    pub fn possible_next_steps(&self) {
        todo!()
    }

    #[must_use]
    pub fn get_visited_services(&self) -> Vec<&Service> {
        self.0.iter().map(JourneyStep::get_service).collect()
    }
}

/// A `Journeys` represents all Journeys a group of users could take through the system
#[derive(Debug, Clone, PartialEq)]
pub struct Journeys {
    journeys: Vec<Journey>,
    goal: Confidence,
}

impl Journeys {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.journeys.iter().all(|j| j.completed_status().is_some())
    }

    #[must_use]
    pub fn step(_all_services: &[Service]) -> bool {
        todo!()
    }
}
