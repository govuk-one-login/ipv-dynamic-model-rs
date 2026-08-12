use crate::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Strength,
    Validity,
    IdentityFraud,
    ActivityHistory,
    Verification,
    Other,
}

impl Row {
    /// Would the CRI appear in a given row
    ///
    /// For example, if the row is `Validity`, and the CRI has a validity score, then it would
    /// appear in that row. If it doesn't have a validity score, then it wouldn't.
    #[must_use]
    pub fn cri_appears_in_row(self, cri: &Cri) -> bool {
        match self {
            Self::Strength => cri.has_strength_score(),
            Self::Validity => cri.has_validity_score(),
            Self::IdentityFraud => cri.has_identity_fraud_score(),
            Self::ActivityHistory => cri.has_activity_history_score(),
            Self::Verification => cri.has_verification_score(),
            Self::Other => !cri.has_score(),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strength => write!(f, "Strength"),
            Self::Validity => write!(f, "Validity"),
            Self::IdentityFraud => write!(f, "Identity Fraud"),
            Self::ActivityHistory => write!(f, "Activity History"),
            Self::Verification => write!(f, "Verification"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::score::{
        ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore, VerificationScore,
    };
    use crate::models::scores::Scores;
    use crate::test_utils::{CreateTestSubject, RandomChoice};

    fn create_cri_with_no_scores() -> Cri {
        let mut cri = Cri::create_test_subject();
        cri.scores = Scores::default();
        cri
    }

    #[test]
    fn test_cri_appears_in_strength() {
        let mut cri = create_cri_with_no_scores();
        assert!(!Row::Strength.cri_appears_in_row(&cri));

        cri.scores.strength = Some(StrengthScore::random_choice());
        assert!(Row::Strength.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_cri_appears_in_validity() {
        let mut cri = create_cri_with_no_scores();
        assert!(!Row::Strength.cri_appears_in_row(&cri));

        cri.scores.validity = Some(ValidityScore::random_choice());
        assert!(Row::Validity.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_cri_appears_in_identity_fraud() {
        let mut cri = create_cri_with_no_scores();
        assert!(!Row::Strength.cri_appears_in_row(&cri));

        cri.scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        assert!(Row::IdentityFraud.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_cri_appears_in_activity_history() {
        let mut cri = create_cri_with_no_scores();
        assert!(!Row::Strength.cri_appears_in_row(&cri));

        cri.scores.activity_history = Some(ActivityHistoryScore::random_choice());
        assert!(Row::ActivityHistory.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_cri_appears_in_verification() {
        let mut cri = create_cri_with_no_scores();
        assert!(!Row::Strength.cri_appears_in_row(&cri));

        cri.scores.verification = Some(VerificationScore::random_choice());
        assert!(Row::Verification.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_cri_appears_in_other() {
        let cri = create_cri_with_no_scores();
        assert!(Row::Other.cri_appears_in_row(&cri));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Row::Strength), "Strength");
        assert_eq!(format!("{}", Row::Validity), "Validity");
        assert_eq!(format!("{}", Row::IdentityFraud), "Identity Fraud");
        assert_eq!(format!("{}", Row::ActivityHistory), "Activity History");
        assert_eq!(format!("{}", Row::Verification), "Verification");
        assert_eq!(format!("{}", Row::Other), "Other");
    }
}
