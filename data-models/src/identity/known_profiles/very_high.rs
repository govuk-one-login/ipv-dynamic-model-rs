use crate::prelude::*;

/// Very high confidence, 1 piece of evidence, profile A (V1A)
pub const V1A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Four),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::Three),
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// Very high confidence, 1 piece of evidence, profile B (V1B)
pub const V1B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Four),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Three),
    },
    None,
    None,
);

/// Very high confidence, 1 piece of evidence, profile C (V1C)
pub const V1C: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
        activity_history: Some(ActivityHistoryScore::One),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Four),
    },
    None,
    None,
);

/// Very high confidence, 1 piece of evidence, profile D (V1D)
pub const V1D: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Four),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::Four),
    },
    None,
    None,
);

/// Very high confidence, 2 pieces of evidence, profile A (V2A)
pub const V2A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: Some(ActivityHistoryScore::Three),
        identity_fraud: Some(IdentityFraudScore::Two),
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

/// Very high confidence, 2 pieces of evidence, profile B (V2B)
pub const V2B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::Two),
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

/// Very high confidence, 2 pieces of evidence, profile C (V2C)
pub const V2C: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Three),
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
    None,
);

/// Very high confidence, 2 pieces of evidence, profile D (V2D)
pub const V2D: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Four),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::Three),
    },
    Some(Scores {
        strength: Some(StrengthScore::Four),
        validity: Some(ValidityScore::Four),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// Very high confidence, 3 pieces of evidence, profile A (V3A)
pub const V3A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Three),
        activity_history: Some(ActivityHistoryScore::Three),
        identity_fraud: Some(IdentityFraudScore::Three),
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
