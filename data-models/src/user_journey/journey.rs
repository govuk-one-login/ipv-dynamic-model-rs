use crate::identity::Proofing;
use crate::prelude::*;
use crate::user_journey::journey_step::JourneyStep;
use crate::user_journey::rule::service_filter::{
    create_ci_filter, create_down_filter, create_visited_filer,
};
use crate::user_journey::rule::service_weights::{
    ServiceWeight, create_sort_by_remaining_capacity,
};

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

    /// Takes the next step in the journey.
    ///
    /// This modifies the current journey, however, if there are any branches in the next step, they
    /// will be returned as a separate `[Vec]`.
    ///
    /// If there are no branches, the returned vector will be empty.
    ///
    /// If the journey has a [`CompletedStatus`], nothing will happen when calling this method.
    #[must_use]
    pub fn step(&mut self, services: &[&Service]) -> Vec<Self> {
        if self.completed_status().is_some() {
            return Vec::new();
        }

        let mut possible_services_weighted = services
            .iter()
            .copied()
            // First remove unusable services
            .filter(create_down_filter(self))
            .filter(create_visited_filer(self))
            .filter(create_ci_filter(self))
            // Next we'll swap to the weighted type for further processing
            .map(ServiceWeight::from)
            .collect();

        create_sort_by_remaining_capacity(1.0)(&mut possible_services_weighted);

        todo!()
    }
}
