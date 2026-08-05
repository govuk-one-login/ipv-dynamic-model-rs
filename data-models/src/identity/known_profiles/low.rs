use crate::prelude::*;

/// Low confidence, 1 piece of evidence, profile A (L1A)
pub const L1A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Two),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::One),
    },
    None,
    None,
);

/// Low confidence, 1 piece of evidence, profile B (L1B)
pub const L1B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::Three),
        validity: Some(ValidityScore::Two),
        activity_history: None,
        identity_fraud: None,
        verification: Some(VerificationScore::One),
    },
    None,
    None,
);

/// Low confidence, 1 piece of evidence, profile C (L1C)
pub const L1C: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: Some(ActivityHistoryScore::Three),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::Two),
    },
    None,
    None,
);

/// Low confidence, 2 pieces of evidence, profile A (L2A)
pub const L2A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::Two),
    },
    Some(Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// Low confidence, 2 pieces of evidence, profile B (L2B)
pub const L2B: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::Two),
        verification: Some(VerificationScore::One),
    },
    Some(Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    None,
);

/// Low confidence, 3 pieces of evidence, profile A (L3A)
pub const L3A: IdentityProfile = IdentityProfile(
    Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: Some(ActivityHistoryScore::Two),
        identity_fraud: Some(IdentityFraudScore::One),
        verification: Some(VerificationScore::One),
    },
    Some(Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
    Some(Scores {
        strength: Some(StrengthScore::One),
        validity: Some(ValidityScore::One),
        activity_history: None,
        identity_fraud: None,
        verification: None,
    }),
);
