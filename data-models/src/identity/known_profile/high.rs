use crate::identity::known_profile::IdentityProfileName;
use crate::prelude::*;

/// High confidence, 1 piece of evidence, profile A (H1A)
pub const H1A: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H1A,
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// High confidence, 1 piece of evidence, profile B (H1B)
pub const H1B: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H1B,
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// High confidence, 1 piece of evidence, profile C (H1C)
pub const H1C: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H1C,
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::Four),
    },
    None,
    None,
);

/// High confidence, 2 pieces of evidence, profile A (H2A)
pub const H2A: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H2A,
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::Three),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// High confidence, 2 pieces of evidence, profile B (H2B)
pub const H2B: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H2B,
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// High confidence, 2 pieces of evidence, profile C (H2C)
pub const H2C: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H2C,
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: Some(ActivityHistoryScore::One),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// High confidence, 2 pieces of evidence, profile D (H2D)
pub const H2D: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H2D,
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// High confidence, 2 pieces of evidence, profile E (H2E)
pub const H2E: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H2E,
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// High confidence, 3 pieces of evidence, profile A (H3A)
pub const H3A: KnownIdentityProfile = KnownIdentityProfile(
    IdentityProfileName::H3A,
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    Some(Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
);
