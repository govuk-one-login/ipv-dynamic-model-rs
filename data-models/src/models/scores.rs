use crate::models::score::{
    ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore, VerificationScore,
};
use crate::models::score_type::ScoreType;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

    fn has_score_of_type(&self, score_type: ScoreType) -> bool {
        match score_type {
            ScoreType::Strength => self.has_strength_score(),
            ScoreType::Validity => self.has_validity_score(),
            ScoreType::Verification => self.has_verification_score(),
            ScoreType::ActivityHistory => self.has_activity_history_score(),
            ScoreType::IdentityFraud => self.has_identity_fraud_score(),
        }
    }

    /// Based on `[ScoreType::order_of_score_importance]`
    #[must_use]
    fn most_important_score_type(&self) -> Option<ScoreType> {
        ScoreType::order_of_score_importance()
            .into_iter()
            .find(|score_type| self.has_score_of_type(*score_type))
    }

    #[must_use]
    fn compare_score_types_and_scores<H: HasScores>(&self, other: &H) -> Ordering {
        match (
            self.most_important_score_type(),
            other.most_important_score_type(),
        ) {
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
            (Some(left_score), Some(right_score)) => {
                let score_type_comparison = left_score.compare_importance_with(right_score);
                if score_type_comparison == Ordering::Equal {
                    // As they're equal it doesn't matter which we check
                    let left = self.scores();
                    let right = other.scores();
                    match left_score {
                        ScoreType::Strength => left.strength.cmp(&right.strength),
                        ScoreType::Validity => left.validity.cmp(&right.validity),
                        ScoreType::Verification => left.verification.cmp(&right.verification),
                        ScoreType::ActivityHistory => {
                            left.activity_history.cmp(&right.activity_history)
                        }
                        ScoreType::IdentityFraud => left.identity_fraud.cmp(&right.identity_fraud),
                    }
                } else {
                    score_type_comparison
                }
            }
        }
    }
}

impl HasScores for Scores {
    fn scores(&self) -> &Scores {
        self
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils {
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
