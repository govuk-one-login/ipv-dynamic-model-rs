use crate::identity::Proofing;
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
    #[must_use]
    pub const fn get_users(&self) -> &Users {
        match self {
            Self::Success(users, _) | Self::CouldNotUse(users, _) | Self::Failure(users, _) => {
                users
            }
        }
    }
}

pub enum CompletedStatus {
    Success(Users, KnownIdentityProfile),
    Failure(Users),
}

/// A `Journey` represents a single path that a user is taking through the system
#[derive(Debug, Clone, PartialEq)]
pub struct Journey {
    journeys: Vec<JourneyStep>,
    goal: Proofing,
}

impl Journey {
    #[must_use]
    pub fn new(goal: Proofing) -> Self {
        Self {
            journeys: Vec::with_capacity(20), // Cheating a bit but this number should be more than the most steps they can take
            goal,
        }
    }

    /// Is the individual service complete
    #[must_use]
    pub fn completed_status(&self) -> Option<CompletedStatus> {
        if let JourneyStep::Failure(users, _) = self.journeys.last()? {
            return Some(CompletedStatus::Failure(users.clone()));
        }
        todo!()
    }

    pub fn possible_next_steps(&self) {
        todo!()
    }

    #[must_use]
    pub fn get_visited_services(&self) -> Vec<&Service> {
        self.journeys.iter().map(JourneyStep::get_service).collect()
    }

    #[must_use]
    pub fn get_unmitigated_cis(&self) -> Vec<&String> {
        self.journeys
            .last()
            .map_or_else(Vec::new, |step| step.get_users().get_unmitigated_cis())
    }
}

/// A `Journeys` represents all Journeys a group of users could take through the system
#[derive(Debug, Clone, PartialEq)]
pub struct Journeys {
    journeys: Vec<Journey>,
    goal: Proofing,
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
