use crate::prelude::{Service, Users};

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
