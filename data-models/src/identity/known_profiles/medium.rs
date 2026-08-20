use crate::prelude::*;

/// Medium confidence, 1 piece of evidence, profile A (M1A)
pub const M1A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Two),
    },
    None,
    None,
);

/// Medium confidence, 1 piece of evidence, profile B (M1B)
pub const M1B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::One),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Two),
    },
    None,
    None,
);

/// Medium confidence, 1 piece of evidence, profile C (M1C)
pub const M1C: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// Medium confidence, 1 piece of evidence, profile D (M1D)
pub const M1D: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// Medium confidence, 2 pieces of evidence, profile A (M2A)
pub const M2A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::Three),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Two),
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

/// Medium confidence, 2 pieces of evidence, profile B (M2B)
pub const M2B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::One),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Two),
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

/// Medium confidence, 2 pieces of evidence, profile C (M2C)
pub const M2C: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: None,
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

/// Medium confidence, 3 pieces of evidence, profile A (M3A)
pub const M3A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Two),
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
