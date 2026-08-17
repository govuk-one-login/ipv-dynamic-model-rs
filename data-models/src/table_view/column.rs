use crate::prelude::*;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub enum RowContent<'s> {
    StartOfCri {
        service: Ref<'s, Service>,
        rowspan: usize,
    },
    ContinuationOfCri {
        service: Ref<'s, Service>,
    },
    Empty,
}

// Ridiculous way to allow comparison due to Ref not implementing PartialEq directly
impl<'s1, 's2> PartialEq<RowContent<'s2>> for RowContent<'s1> {
    fn eq(&self, other: &RowContent<'s2>) -> bool {
        match (self, other) {
            (
                RowContent::StartOfCri {
                    service: self_service,
                    rowspan: self_rowspan,
                },
                RowContent::StartOfCri {
                    service: other_service,
                    rowspan: other_rowspan,
                },
            ) => self_rowspan == other_rowspan && self_service.deref() == other_service.deref(),
            (
                RowContent::ContinuationOfCri {
                    service: self_service,
                },
                RowContent::ContinuationOfCri {
                    service: other_service,
                },
            ) => self_service.deref() == other_service.deref(),
            (RowContent::Empty, RowContent::Empty) => true,
            _ => false,
        }
    }
}

impl<'s> RowContent<'s> {
    #[must_use]
    pub fn get_service(&'s self) -> Option<Ref<'s, Service>> {
        match self {
            Self::StartOfCri { service, .. } | Self::ContinuationOfCri { service } => {
                Some(Ref::clone(service))
            }
            Self::Empty => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Column {
    pub strength: Option<Rc<RefCell<Service>>>,
    pub validity: Option<Rc<RefCell<Service>>>,
    pub activity_history: Option<Rc<RefCell<Service>>>,
    pub identity_fraud: Option<Rc<RefCell<Service>>>,
    pub verification: Option<Rc<RefCell<Service>>>,
    pub other: Option<Rc<RefCell<Service>>>,
}

impl Column {
    /// Checks if the column already contains the given CRI
    #[must_use]
    pub fn contains_service(&self, service: &Service) -> bool {
        // Closure to simplify having to do the same check for every row
        let is_cri_in_row = |row: &Option<Rc<RefCell<Service>>>| -> bool {
            row.as_ref().is_some_and(|s| {
                let inner = s.borrow();
                inner.deref() == service
            })
        };

        // Note: keep this in row order
        is_cri_in_row(&self.strength)
            || is_cri_in_row(&self.validity)
            || is_cri_in_row(&self.verification)
            || is_cri_in_row(&self.activity_history)
            || is_cri_in_row(&self.identity_fraud)
            || is_cri_in_row(&self.other)
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

    pub fn add_service(&mut self, service: Rc<RefCell<Service>>) {
        if !service.borrow().scores.has_score() {
            self.other = Some(service);
            return;
        }
        if service.borrow().has_strength_score() {
            self.strength = Some(service.clone());
        }
        if service.borrow().has_validity_score() {
            self.validity = Some(service.clone());
        }
        if service.borrow().has_activity_history_score() {
            self.activity_history = Some(service.clone());
        }
        if service.borrow().has_identity_fraud_score() {
            self.identity_fraud = Some(service.clone());
        }
        if service.borrow().has_verification_score() {
            self.verification = Some(service.clone());
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
            && self
                .get_service_at(*previous_row)
                .is_some_and(|in_row| in_row.deref() == service.deref())
        {
            return RowContent::ContinuationOfCri { service };
        }

        // Otherwise we have a new CRI for this column, so we need to see how many rows it
        // continues on for
        let rowspan = Self::row_order()[row_pos..]
            .iter()
            .copied()
            .take_while(|r| {
                self.get_service_at(*r)
                    .is_some_and(|in_row| in_row.deref() == service.deref())
            })
            .count();
        RowContent::StartOfCri { service, rowspan }
    }

    #[must_use]
    fn get_service_at(&self, row: Row) -> Option<Ref<'_, Service>> {
        match row {
            Row::Strength => self.strength.as_ref().map(|inner| inner.borrow()),
            Row::Validity => self.validity.as_ref().map(|inner| inner.borrow()),
            Row::ActivityHistory => self.activity_history.as_ref().map(|inner| inner.borrow()),
            Row::IdentityFraud => self.identity_fraud.as_ref().map(|inner| inner.borrow()),
            Row::Verification => self.verification.as_ref().map(|inner| inner.borrow()),
            Row::Other => self.other.as_ref().map(|inner| inner.borrow()),
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
        let cri = create_cri_with_no_scores();
        let service = Rc::new(RefCell::new(Service::new(cri)));
        assert!(!column.contains_service(&service.borrow()));
        column.add_service(service.clone());
        assert!(column.contains_service(&service.borrow()));
    }

    #[test]
    fn test_is_row_filled() {
        let mut column = Column::default();

        let mut cri = create_cri_with_no_scores();
        cri.scores.strength = Some(StrengthScore::random_choice());
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::Strength));
        column.add_service(service);
        assert!(column.is_row_filled(Row::Strength));

        let mut cri = create_cri_with_no_scores();
        cri.scores.validity = Some(ValidityScore::random_choice());
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::Validity));
        column.add_service(service);
        assert!(column.is_row_filled(Row::Validity));

        let mut cri = create_cri_with_no_scores();
        cri.scores.activity_history = Some(ActivityHistoryScore::random_choice());
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::ActivityHistory));
        column.add_service(service);
        assert!(column.is_row_filled(Row::ActivityHistory));

        let mut cri = create_cri_with_no_scores();
        cri.scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::IdentityFraud));
        column.add_service(service);
        assert!(column.is_row_filled(Row::IdentityFraud));

        let mut cri = create_cri_with_no_scores();
        cri.scores.verification = Some(VerificationScore::random_choice());
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::Verification));
        column.add_service(service);
        assert!(column.is_row_filled(Row::Verification));

        let cri = create_cri_with_no_scores();
        let service = Rc::new(RefCell::new(Service::new(cri)));

        assert!(!column.is_row_filled(Row::Other));
        column.add_service(service);
        assert!(column.is_row_filled(Row::Other));
    }

    #[test]
    fn test_get_row() {
        let mut strength_and_validity = create_cri_with_no_scores();
        strength_and_validity.scores.strength = Some(StrengthScore::random_choice());
        strength_and_validity.scores.validity = Some(ValidityScore::random_choice());
        let strength_and_validity = Rc::new(RefCell::new(Service::new(strength_and_validity)));

        let mut identity_fraud = create_cri_with_no_scores();
        identity_fraud.scores.identity_fraud = Some(IdentityFraudScore::random_choice());
        let identity_fraud = Rc::new(RefCell::new(Service::new(identity_fraud)));

        let other = create_cri_with_no_scores();
        let other = Rc::new(RefCell::new(Service::new(other)));

        let mut column = Column::default();
        column.add_service(strength_and_validity.clone());
        column.add_service(identity_fraud.clone());
        column.add_service(other.clone());

        assert_eq!(
            column.get_row(Row::Strength),
            RowContent::StartOfCri {
                service: strength_and_validity.borrow(),
                rowspan: 2
            }
        );
        assert_eq!(
            column.get_row(Row::Validity),
            RowContent::ContinuationOfCri {
                service: strength_and_validity.borrow()
            }
        );

        assert_eq!(column.get_row(Row::Verification), RowContent::Empty);
        assert_eq!(column.get_row(Row::ActivityHistory), RowContent::Empty);
        assert_eq!(
            column.get_row(Row::IdentityFraud),
            RowContent::StartOfCri {
                service: identity_fraud.borrow(),
                rowspan: 1
            }
        );
        assert_eq!(
            column.get_row(Row::Other),
            RowContent::StartOfCri {
                service: other.borrow(),
                rowspan: 1
            }
        );
    }
}
