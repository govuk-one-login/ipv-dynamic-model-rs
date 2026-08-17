use std::cell::RefCell;
use crate::models::cri::Cri;
use crate::models::scores::HasScores;
use crate::table_view::column::{Column, RowContent};
use crate::table_view::row::Row;
use std::rc::Rc;
use crate::prelude::Service;

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    #[must_use]
    pub fn new(mut services: Vec<Rc<RefCell<Service>>>) -> Self {
        // Sort the CRIs by number of scores they contain
        services.sort_by_key(|service| service.borrow().score_count());
        services.reverse();

        // We'll prevent reallocation by allocating for worst case scenario
        let mut table = Self {
            columns: Vec::with_capacity(services.len()),
        };

        for row in Column::row_order() {
            for service in &services {
                if table.contains_service(&service.borrow()) || !row.cri_appears_in_row(&service.borrow()) {
                    continue;
                }
                table.get_first_available_column(row).add_service(service.clone());
            }
        }

        table
    }

    /// Does the table already have an entry for this CRI anywhere within it
    #[must_use]
    fn contains_service(&self, cri: &Service) -> bool {
        self.columns.iter().any(|column| column.contains_service(cri))
    }

    // ToDo: This needs to accept CRI so that it can check all rows a CRI would appear in
    #[must_use]
    fn get_first_available_column(&mut self, row: Row) -> &mut Column {
        // This function is a bit weird to get around lifetime woes

        // If there is no column available, add one
        if !self.columns.iter().any(|c| !c.is_row_filled(row)) {
            self.columns.push(Column::default());
        }
        // Return the first available column
        self.columns
            .iter_mut()
            .find(|c| !c.is_row_filled(row))
            .unwrap() // This will always be ok because we'll have just added an empty column
    }

    #[must_use]
    pub fn get_row(&self, row: Row) -> Vec<RowContent> {
        self.columns
            .iter()
            .map(|column| column.get_row(row))
            .collect()
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
    use rand::prelude::SliceRandom;

    fn create_service_for_rows(name: &str, rows: &[Row]) -> Rc<RefCell<Service>> {
        let mut cri = Cri::create_test_subject();
        cri.name = name.to_string();
        cri.scores = Scores::default();

        for row in rows {
            match row {
                Row::Strength => cri.scores.strength = Some(StrengthScore::random_choice()),
                Row::Validity => cri.scores.validity = Some(ValidityScore::random_choice()),
                Row::IdentityFraud => {
                    cri.scores.identity_fraud = Some(IdentityFraudScore::random_choice())
                }
                Row::ActivityHistory => {
                    cri.scores.activity_history = Some(ActivityHistoryScore::random_choice())
                }
                Row::Verification => {
                    cri.scores.verification = Some(VerificationScore::random_choice())
                }
                Row::Other => cri.scores = Scores::default(),
            }
        }
        Rc::new(RefCell::new(Service::new(cri)))
    }

    #[test]
    fn test_all_the_things() {
        // This should populate the first column fully
        let service1 = create_service_for_rows(
            "service1",
            &[Row::Strength, Row::IdentityFraud, Row::ActivityHistory],
        );
        let service2 = create_service_for_rows("service2", &[Row::Validity]);
        let service3 = create_service_for_rows("service3", &[Row::Verification]);
        let service4 = create_service_for_rows("service4", &[Row::Other]);

        // This should appear on column 2
        let service5 = create_service_for_rows("service5", &[Row::Strength, Row::IdentityFraud]);

        // This should appear on column 3
        let service6 = create_service_for_rows("service6", &[Row::IdentityFraud]);

        let mut services = vec![
            service1.clone(),
            service2.clone(),
            service3.clone(),
            service4.clone(),
            service5.clone(),
            service6.clone(),
        ];
        services.shuffle(&mut rand::rng());

        let table = Table::new(services);

        let strength_row: Vec<_> = table
            .get_row(Row::Strength)
            .into_iter()
            .map(|content| content.get_service().map(|service| service.clone()))
            .collect();
        assert_eq!(
            strength_row,
            vec![Some(service1.borrow().clone()), Some(service5.borrow().clone()), None]
        );
    }
}
