use crate::models::scores::Scores;
use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct IdentityProfile(pub Scores, pub Option<Scores>, pub Option<Scores>);

impl IdentityProfile {
    /// All profiles that meet p4 assurance, ie, all Very High profiles
    #[must_use]
    pub const fn p4_profiles() -> [Self; 9] {
        [
            // Very High
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A,
        ]
    }

    /// All profiles that meet p3 assurance, ie, all High and Very High profiles
    #[must_use]
    pub const fn p3_profiles() -> [Self; 18] {
        [
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
        ]
    }

    /// All profiles that meet p2 assurance, ie, all Medium, High and Very High profiles
    #[must_use]
    pub const fn p2_profiles() -> [Self; 26] {
        [
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
            M1A, M1B, M1C, M1D, M2A, M2B, M2C, M3A, // Medium
        ]
    }

    /// All profiles that meet p1 assurance, ie, all Low, Medium, High and Very High profiles
    #[must_use]
    pub const fn p1_profiles() -> [Self; 32] {
        [
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
            M1A, M1B, M1C, M1D, M2A, M2B, M2C, M3A, // Medium
            L1A, L1B, L1C, L2A, L2B, L3A, // Low
        ]
    }
}
