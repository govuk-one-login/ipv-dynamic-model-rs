pub mod identity;
pub mod models;
pub mod table_view;

#[cfg(test)]
mod test_utils;

pub mod prelude {
    pub use super::identity::{identity_profile::IdentityProfile, known_profiles::*};
    pub use super::models::{
        attribute::Attribute,
        claim::Claim,
        cri::Cri,
        requests_per_second::RequestsPerSecond,
        score::{
            ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore,
            VerificationScore,
        },
        scores::{HasScores, Scores},
        user_requirement::UserRequirement,
    };
    pub use super::table_view::{
        column::{Column, RowContent},
        row::Row,
        service::Service,
        table::Table,
    };
}
