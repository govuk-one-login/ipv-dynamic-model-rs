use std::fmt;
use crate::models::cri::Cri;
use crate::models::scores::HasScores;
use crate::table_view::row::Row;
use std::rc::Rc;

pub const ROW_ORDER: [Row; 6] = [
    Row::Strength,
    Row::Validity,
    Row::IdentityFraud,
    Row::ActivityHistory,
    Row::Verification,
    Row::Other,
];

#[derive(Debug, Clone, Default)]
pub struct Column {
    pub strength: Option<Rc<Cri>>,
    pub validity: Option<Rc<Cri>>,
    pub identity_fraud: Option<Rc<Cri>>,
    pub activity_history: Option<Rc<Cri>>,
    pub verification: Option<Rc<Cri>>,
    pub other: Option<Rc<Cri>>,
}

impl Column {
    #[must_use]
    pub fn contains_cri(&self, cri: &Rc<Cri>) -> bool {
        self.strength == Some(cri.clone())
            || self.validity == Some(cri.clone())
            || self.identity_fraud == Some(cri.clone())
            || self.activity_history == Some(cri.clone())
            || self.verification == Some(cri.clone())
            || self.other == Some(cri.clone())
    }

    /// In this column, does the given row already have an entry?
    #[must_use]
    pub const fn is_row_filled(&self, row: Row) -> bool {
        match row {
            Row::Strength => self.strength.is_some(),
            Row::Validity => self.validity.is_some(),
            Row::IdentityFraud => self.identity_fraud.is_some(),
            Row::ActivityHistory => self.activity_history.is_some(),
            Row::Verification => self.verification.is_some(),
            Row::Other => self.other.is_some(),
        }
    }

    pub fn add_cri(&mut self, cri: &Rc<Cri>) {
        if !cri.scores.has_score() {
            self.other = Some(cri.clone());
            return;
        }
        if cri.has_strength_score() {
            self.strength = Some(cri.clone());
        }
        if cri.has_validity_score() {
            self.validity = Some(cri.clone());
        }
        if cri.has_identity_fraud_score() {
            self.identity_fraud = Some(cri.clone());
        }
        if cri.has_activity_history_score() {
            self.activity_history = Some(cri.clone());
        }
        if cri.has_verification_score() {
            self.verification = Some(cri.clone());
        }
    }

    /// Get the CRI at a given row in this column
    pub fn get_row(&self, row: Row) -> Option<Rc<Cri>> {
        match row {
            Row::Strength => self.strength.clone(),
            Row::Validity => self.validity.clone(),
            Row::IdentityFraud => self.identity_fraud.clone(),
            Row::ActivityHistory => self.activity_history.clone(),
            Row::Verification => self.verification.clone(),
            Row::Other => self.other.clone(),
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
    fn test_contains_cri() {
        let mut column = Column::default();
        let cri = Rc::new(create_cri_with_no_scores());
        assert!(!column.contains_cri(&cri));
        column.add_cri(&cri);
        assert!(column.contains_cri(&cri));
    }

    #[test]
    fn test_is_row_filled() {
        let mut column = Column::default();

        let mut cri = create_cri_with_no_scores();
        cri.scores.strength = Some(StrengthScore::random_choice());

        assert!(!column.is_row_filled(Row::Strength));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::Strength));

        let mut cri = create_cri_with_no_scores();
        cri.scores.validity = Some(ValidityScore::random_choice());

        assert!(!column.is_row_filled(Row::Validity));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::Validity));

        let mut cri = create_cri_with_no_scores();
        cri.scores.identity_fraud = Some(IdentityFraudScore::random_choice());

        assert!(!column.is_row_filled(Row::IdentityFraud));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::IdentityFraud));

        let mut cri = create_cri_with_no_scores();
        cri.scores.activity_history = Some(ActivityHistoryScore::random_choice());

        assert!(!column.is_row_filled(Row::ActivityHistory));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::ActivityHistory));

        let mut cri = create_cri_with_no_scores();
        cri.scores.verification = Some(VerificationScore::random_choice());

        assert!(!column.is_row_filled(Row::Verification));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::Verification));

        let cri = create_cri_with_no_scores();

        assert!(!column.is_row_filled(Row::Other));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::Other));
    }

    #[test]
    fn test_get_row() {
        let mut strength_and_verification = create_cri_with_no_scores();
        strength_and_verification.scores.strength = Some(StrengthScore::random_choice());
        strength_and_verification.scores.verification = Some(VerificationScore::random_choice());
        let strength_and_verification = Rc::new(strength_and_verification);

        let mut identity_fraud = create_cri_with_no_scores();
        identity_fraud.scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        let identity_fraud = Rc::new(identity_fraud);

        let other = create_cri_with_no_scores();
        let other = Rc::new(other);

        let mut column = Column::default();
        column.add_cri(&strength_and_verification);
        column.add_cri(&identity_fraud);
        column.add_cri(&other);

        assert_eq!(
            column.get_row(Row::Strength),
            Some(strength_and_verification.clone())
        );
        assert_eq!(column.get_row(Row::Validity), None);
        assert_eq!(column.get_row(Row::IdentityFraud), Some(identity_fraud));
        assert_eq!(column.get_row(Row::ActivityHistory), None);
        assert_eq!(
            column.get_row(Row::Verification),
            Some(strength_and_verification)
        );
        assert_eq!(column.get_row(Row::Other), Some(other));
    }
}
