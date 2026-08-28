mod low;
pub use low::*;
mod medium;
pub use medium::*;
mod high;
pub use high::*;
mod very_high;
pub use very_high::*;

use crate::models::scores::Scores;
use core::fmt;
use crate::identity::Proofing;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IdentityProfileName {
    // Very High
    V1A,
    V1B,
    V1C,
    V1D,
    V2A,
    V2B,
    V2C,
    V2D,
    V3A,
    // High
    H1A,
    H1B,
    H1C,
    H2A,
    H2B,
    H2C,
    H2D,
    H2E,
    H3A,
    // Medium
    M1A,
    M1B,
    M1C,
    M1D,
    M2A,
    M2B,
    M2C,
    M3A,
    // Low
    L1A,
    L1B,
    L1C,
    L2A,
    L2B,
    L3A,
}

impl fmt::Display for IdentityProfileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1A => write!(f, "V1A"),
            Self::V1B => write!(f, "V1B"),
            Self::V1C => write!(f, "V1C"),
            Self::V1D => write!(f, "V1D"),
            Self::V2A => write!(f, "V2A"),
            Self::V2B => write!(f, "V2B"),
            Self::V2C => write!(f, "V2C"),
            Self::V2D => write!(f, "V2D"),
            Self::V3A => write!(f, "V3A"),
            Self::H1A => write!(f, "H1A"),
            Self::H1B => write!(f, "H1B"),
            Self::H1C => write!(f, "H1C"),
            Self::H2A => write!(f, "H2A"),
            Self::H2B => write!(f, "H2B"),
            Self::H2C => write!(f, "H2C"),
            Self::H2D => write!(f, "H2D"),
            Self::H2E => write!(f, "H2E"),
            Self::H3A => write!(f, "H3A"),
            Self::M1A => write!(f, "M1A"),
            Self::M1B => write!(f, "M1B"),
            Self::M1C => write!(f, "M1C"),
            Self::M1D => write!(f, "M1D"),
            Self::M2A => write!(f, "M2A"),
            Self::M2B => write!(f, "M2B"),
            Self::M2C => write!(f, "M2C"),
            Self::M3A => write!(f, "M3A"),
            Self::L1A => write!(f, "L1A"),
            Self::L1B => write!(f, "L1B"),
            Self::L1C => write!(f, "L1C"),
            Self::L2A => write!(f, "L2A"),
            Self::L2B => write!(f, "L2B"),
            Self::L3A => write!(f, "L3A"),
        }
    }
}

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

    pub const fn profiles_of(proofing: Proofing) -> &'static [Self] {
        match proofing {
            Proofing::P1 => Self::p1_profiles(),
            Proofing::P2 => Self::p2_profiles(),
            Proofing::P3 => Self::p3_profiles(),
            Proofing::P4 => Self::p4_profiles(),
        }
    }
}

impl From<IdentityProfileName> for KnownIdentityProfile {
    fn from(value: IdentityProfileName) -> Self {
        match value {
            IdentityProfileName::V1A => V1A,
            IdentityProfileName::V1B => V1B,
            IdentityProfileName::V1C => V1C,
            IdentityProfileName::V1D => V1D,
            IdentityProfileName::V2A => V2A,
            IdentityProfileName::V2B => V2B,
            IdentityProfileName::V2C => V2C,
            IdentityProfileName::V2D => V2D,
            IdentityProfileName::V3A => V3A,
            IdentityProfileName::H1A => H1A,
            IdentityProfileName::H1B => H1B,
            IdentityProfileName::H1C => H1C,
            IdentityProfileName::H2A => H2A,
            IdentityProfileName::H2B => H2B,
            IdentityProfileName::H2C => H2C,
            IdentityProfileName::H2D => H2D,
            IdentityProfileName::H2E => H2E,
            IdentityProfileName::H3A => H3A,
            IdentityProfileName::M1A => M1A,
            IdentityProfileName::M1B => M1B,
            IdentityProfileName::M1C => M1C,
            IdentityProfileName::M1D => M1D,
            IdentityProfileName::M2A => M2A,
            IdentityProfileName::M2B => M2B,
            IdentityProfileName::M2C => M2C,
            IdentityProfileName::M3A => M3A,
            IdentityProfileName::L1A => L1A,
            IdentityProfileName::L1B => L1B,
            IdentityProfileName::L1C => L1C,
            IdentityProfileName::L2A => L2A,
            IdentityProfileName::L2B => L2B,
            IdentityProfileName::L3A => L3A,
        }
    }
}
