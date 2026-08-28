use crate::identity::Proofing;
use crate::prelude::Service;
use crate::user_journey::journey::Journey;

/// A `Journeys` represents all Journeys a group of users could take through the system
#[derive(Debug, Clone, PartialEq)]
pub struct Journeys {
    journeys: Vec<Journey>,
    goal: Proofing,
}

impl Journeys {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.journeys.iter().all(Journey::is_complete)
    }

    #[must_use]
    pub fn step(_all_services: &[Service]) -> bool {
        todo!()
    }
}
