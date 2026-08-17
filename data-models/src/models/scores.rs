use crate::models::score::{
    ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore, VerificationScore,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Scores {
    #[serde(rename = "strengthScore")]
    pub strength: Option<StrengthScore>,
    #[serde(rename = "validityScore")]
    pub validity: Option<ValidityScore>,
    #[serde(rename = "activityHistoryScore")]
    pub activity_history: Option<ActivityHistoryScore>,
    #[serde(rename = "identityFraudScore")]
    pub identity_fraud: Option<IdentityFraudScore>,
    #[serde(rename = "verificationScore")]
    pub verification: Option<VerificationScore>,
}

pub trait HasScores {
    fn scores(&self) -> &Scores;

    fn has_strength_score(&self) -> bool {
        self.scores().strength.is_some()
    }

    fn has_validity_score(&self) -> bool {
        self.scores().validity.is_some()
    }

    fn has_activity_history_score(&self) -> bool {
        self.scores().activity_history.is_some()
    }

    fn has_identity_fraud_score(&self) -> bool {
        self.scores().identity_fraud.is_some()
    }

    fn has_verification_score(&self) -> bool {
        self.scores().verification.is_some()
    }

    #[must_use]
    fn has_score(&self) -> bool {
        self.score_count() > 0
    }

    /// How many types of scores does this CRI produce?
    #[must_use]
    fn score_count(&self) -> usize {
        [
            self.has_strength_score(),
            self.has_validity_score(),
            self.has_identity_fraud_score(),
            self.has_activity_history_score(),
            self.has_verification_score(),
        ]
        .iter()
        .copied()
        .filter(|score| *score)
        .count()
    }
}

impl HasScores for Scores {
    fn scores(&self) -> &Scores {
        self
    }
}

#[cfg(feature = "test-utils")]
pub mod tests_utils {
    use super::*;
    use crate::test_utils::{CreateTestSubject, RandomChoice};

    impl CreateTestSubject for Scores {
        fn create_test_subject() -> Self {
            Self {
                strength: StrengthScore::random_choice_option(0.3),
                validity: ValidityScore::random_choice_option(0.3),
                activity_history: ActivityHistoryScore::random_choice_option(0.3),
                identity_fraud: IdentityFraudScore::random_choice_option(0.3),
                verification: VerificationScore::random_choice_option(0.3),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::RandomChoice;

    #[test]
    fn test_has_score() {
        let mut scores = Scores::default();
        assert!(!scores.has_score());
        scores.strength = Some(StrengthScore::random_choice());
        assert!(scores.has_score());

        let mut scores = Scores::default();
        assert!(!scores.has_score());
        scores.validity = Some(ValidityScore::random_choice());
        assert!(scores.has_score());

        let mut scores = Scores::default();
        assert!(!scores.has_score());
        scores.activity_history = Some(ActivityHistoryScore::random_choice());
        assert!(scores.has_score());

        let mut scores = Scores::default();
        assert!(!scores.has_score());
        scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        assert!(scores.has_score());

        let mut scores = Scores::default();
        assert!(!scores.has_score());
        scores.verification = Some(VerificationScore::random_choice());
        assert!(scores.has_score());
    }

    #[test]
    fn test_get_scores() {
        let mut scores = Scores::default();
        assert_eq!(scores.score_count(), 0);

        scores.strength = Some(StrengthScore::random_choice());
        assert_eq!(scores.score_count(), 1);

        scores.validity = Some(ValidityScore::random_choice());
        assert_eq!(scores.score_count(), 2);

        scores.activity_history = Some(ActivityHistoryScore::random_choice());
        assert_eq!(scores.score_count(), 3);

        scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        assert_eq!(scores.score_count(), 4);

        scores.verification = Some(VerificationScore::random_choice());
        assert_eq!(scores.score_count(), 5);
    }
}
