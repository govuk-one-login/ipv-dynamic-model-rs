use crate::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum RowContent {
    StartOfCri { service: Service, rowspan: usize },
    ContinuationOfCri { service: Service },
    Empty,
}

impl RowContent {
    #[must_use]
    pub fn get_service(&self) -> Option<Service> {
        match self {
            Self::StartOfCri { service, .. } | Self::ContinuationOfCri { service } => {
                Some(service.clone())
            }
            Self::Empty => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Column {
    pub strength: Option<Service>,
    pub validity: Option<Service>,
    pub activity_history: Option<Service>,
    pub identity_fraud: Option<Service>,
    pub verification: Option<Service>,
    pub other: Option<Service>,
}

impl Column {
    /// Checks if the column already contains the given CRI
    #[must_use]
    pub fn contains_cri(&self, cri: &Rc<Cri>) -> bool {
        // Closure to simplify having to do the same check for every row
        let is_cri_in_row = |cri: &Rc<Cri>, row: &Option<Service>| -> bool {
            row.as_ref().is_some_and(|service| service == cri.as_ref())
        };

        is_cri_in_row(cri, &self.strength)
            || is_cri_in_row(cri, &self.validity)
            || is_cri_in_row(cri, &self.activity_history)
            || is_cri_in_row(cri, &self.identity_fraud)
            || is_cri_in_row(cri, &self.verification)
            || is_cri_in_row(cri, &self.other)
    }

    /// In this column, does the given row already have an entry?
    #[must_use]
    pub const fn is_row_filled(&self, row: Row) -> bool {
        match row {
            Row::Strength => self.strength.is_some(),
            Row::Validity => self.validity.is_some(),
            Row::ActivityHistory => self.activity_history.is_some(),
            Row::IdentityFraud => self.identity_fraud.is_some(),
            Row::Verification => self.verification.is_some(),
            Row::Other => self.other.is_some(),
        }
    }

    pub fn add_cri(&mut self, cri: &Rc<Cri>) {
        if !cri.scores.has_score() {
            self.other = Some(Service::new(cri.clone()));
            return;
        }
        if cri.has_strength_score() {
            self.strength = Some(Service::new(cri.clone()));
        }
        if cri.has_validity_score() {
            self.validity = Some(Service::new(cri.clone()));
        }
        if cri.has_activity_history_score() {
            self.activity_history = Some(Service::new(cri.clone()));
        }
        if cri.has_identity_fraud_score() {
            self.identity_fraud = Some(Service::new(cri.clone()));
        }
        if cri.has_verification_score() {
            self.verification = Some(Service::new(cri.clone()));
        }
    }

    /// Get the CRI at a given row in this column
    ///
    /// # Panics
    ///
    /// If a row is requested that is not in `[Self::row_order()]`. Every row needs to appear in
    /// that list.
    #[must_use]
    pub fn get_row(&self, row: Row) -> RowContent {
        let service = self.get_service_at(row);

        // If there is nothing in this row its considered empty
        let Some(service) = service else {
            return RowContent::Empty;
        };

        // If the previous row contains the same thing, this is a continuation of that row
        let row_pos = Self::row_order()
            .iter()
            .position(|r| r == &row)
            .expect("Somehow a row has been missed from the row_order, this is not recoverable");

        if row_pos > 0
            && let Some(previous_row) = Self::row_order().get(row_pos - 1)
            && self.get_service_at(*previous_row).as_ref() == Some(&service)
        {
            return RowContent::ContinuationOfCri { service };
        }

        // Otherwise we have a new CRI for this column, so we need to see how many rows it
        // continues on for
        let rowspan = Self::row_order()[row_pos..]
            .iter()
            .copied()
            .take_while(|r| self.get_service_at(*r).as_ref() == Some(&service))
            .count();
        RowContent::StartOfCri { service, rowspan }
    }

    #[must_use]
    fn get_service_at(&self, row: Row) -> Option<Service> {
        match row {
            Row::Strength => self.strength.clone(),
            Row::Validity => self.validity.clone(),
            Row::ActivityHistory => self.activity_history.clone(),
            Row::IdentityFraud => self.identity_fraud.clone(),
            Row::Verification => self.verification.clone(),
            Row::Other => self.other.clone(),
        }
    }

    #[must_use]
    pub const fn row_order() -> [Row; 6] {
        [
            Row::Strength,
            Row::Validity,
            Row::Verification,
            Row::ActivityHistory,
            Row::IdentityFraud,
            Row::Other,
        ]
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
        cri.scores.activity_history = Some(ActivityHistoryScore::random_choice());

        assert!(!column.is_row_filled(Row::ActivityHistory));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::ActivityHistory));

        let mut cri = create_cri_with_no_scores();
        cri.scores.identity_fraud = Some(IdentityFraudScore::random_choice());

        assert!(!column.is_row_filled(Row::IdentityFraud));
        column.add_cri(&Rc::new(cri));
        assert!(column.is_row_filled(Row::IdentityFraud));

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
        let mut strength_and_validity = create_cri_with_no_scores();
        strength_and_validity.scores.strength = Some(StrengthScore::random_choice());
        strength_and_validity.scores.validity = Some(ValidityScore::random_choice());
        let strength_and_validity = Rc::new(strength_and_validity);

        let mut identity_fraud = create_cri_with_no_scores();
        identity_fraud.scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        let identity_fraud = Rc::new(identity_fraud);

        let other = create_cri_with_no_scores();
        let other = Rc::new(other);

        let mut column = Column::default();
        column.add_cri(&strength_and_validity);
        column.add_cri(&identity_fraud);
        column.add_cri(&other);

        assert_eq!(
            column.get_row(Row::Strength),
            RowContent::StartOfCri {
                service: Service::new(strength_and_validity.clone()),
                rowspan: 2
            }
        );
        assert_eq!(
            column.get_row(Row::Validity),
            RowContent::ContinuationOfCri {
                service: strength_and_validity.into()
            }
        );

        assert_eq!(column.get_row(Row::Verification), RowContent::Empty);
        assert_eq!(column.get_row(Row::ActivityHistory), RowContent::Empty);
        assert_eq!(
            column.get_row(Row::IdentityFraud),
            RowContent::StartOfCri {
                service: Service::new(identity_fraud),
                rowspan: 1
            }
        );
        assert_eq!(
            column.get_row(Row::Other),
            RowContent::StartOfCri {
                service: Service::new(other),
                rowspan: 1
            }
        );
    }
}
