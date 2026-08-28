use crate::prelude::Service;

/// Represents a single step of a journey, not the entire journey. A failed step or successful step
/// still allows the journey to continue
#[derive(Debug, Clone, PartialEq)]
pub enum JourneyStep {
    Success(Service),
    CouldNotUse(Service),
    Failure(Service),
}

impl JourneyStep {
    #[must_use]
    pub const fn get_service(&self) -> &Service {
        match self {
            Self::Success(service) | Self::CouldNotUse(service) | Self::Failure(service) => service,
        }
    }
}
