mod low;
pub use low::*;
mod medium;
pub use medium::*;
mod high;
pub use high::*;
mod very_high;
pub use very_high::*;

use crate::identity::Proofing;
use crate::identity::profile_name::IdentityProfileName;
use crate::models::scores::Scores;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct KnownIdentityProfile(
    pub IdentityProfileName,
    pub Scores,
    pub Option<Scores>,
    pub Option<Scores>,
);

impl KnownIdentityProfile {
    /// All profiles that meet p4 assurance, ie, all Very High profiles
    #[must_use]
    pub const fn p4_profiles() -> &'static [Self; 9] {
        &[
            // Very High
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A,
        ]
    }

    /// All profiles that meet p3 assurance, ie, all High and Very High profiles
    #[must_use]
    pub const fn p3_profiles() -> &'static [Self; 18] {
        &[
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
        ]
    }

    /// All profiles that meet p2 assurance, ie, all Medium, High and Very High profiles
    #[must_use]
    pub const fn p2_profiles() -> &'static [Self; 26] {
        &[
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
            M1A, M1B, M1C, M1D, M2A, M2B, M2C, M3A, // Medium
        ]
    }

    /// All profiles that meet p1 assurance, ie, all Low, Medium, High and Very High profiles
    #[must_use]
    pub const fn p1_profiles() -> &'static [Self; 32] {
        &[
            V1A, V1B, V1C, V1D, V2A, V2B, V2C, V2D, V3A, // Very High
            H1A, H1B, H1C, H2A, H2B, H2C, H2D, H2E, H3A, // High
            M1A, M1B, M1C, M1D, M2A, M2B, M2C, M3A, // Medium
            L1A, L1B, L1C, L2A, L2B, L3A, // Low
        ]
    }

    #[must_use]
    pub const fn profiles_of(proofing: Proofing) -> &'static [Self] {
        match proofing {
            Proofing::P1 => Self::p1_profiles(),
            Proofing::P2 => Self::p2_profiles(),
            Proofing::P3 => Self::p3_profiles(),
            Proofing::P4 => Self::p4_profiles(),
        }
    }

    #[must_use]
    pub fn is_profile_of(self, proofing: Proofing) -> bool {
        Self::profiles_of(proofing).contains(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiles_are_inclusive() {
        // Cheating a bit to do a double test as `profiles_of` is a tiny abstraction that also needs
        // testing
        let p1 = KnownIdentityProfile::profiles_of(Proofing::P1);
        let p2 = KnownIdentityProfile::profiles_of(Proofing::P2);
        let p3 = KnownIdentityProfile::profiles_of(Proofing::P3);
        let p4 = KnownIdentityProfile::profiles_of(Proofing::P4);

        // Transitively if all of p4 is in p3 then it is also in p2 and p1
        assert!(p4.iter().all(|p4_profile| p3.contains(p4_profile)));
        assert!(p3.iter().all(|p3_profile| p2.contains(p3_profile)));
        assert!(p2.iter().all(|p2_profile| p1.contains(p2_profile)));
    }
}
