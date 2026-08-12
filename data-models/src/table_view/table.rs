use crate::models::cri::Cri;
use crate::models::scores::HasScores;
use crate::table_view::column::Column;
use crate::table_view::row::Row;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    #[must_use]
    pub fn new(mut cris: Vec<Rc<Cri>>) -> Self {
        // Sort the CRIs by number of scores they contain
        cris.sort_by_key(|cri| cri.score_count());
        cris.reverse();

        // We'll prevent reallocation by allocating for worst case scenario
        let mut table = Self {
            columns: Vec::with_capacity(cris.len()),
        };

        for row in Column::row_order() {
            for cri in &cris {
                if table.contains_cri(cri) || !row.cri_appears_in_row(cri) {
                    continue;
                }
                table.get_first_available_column(row).add_cri(cri);
            }
        }

        table
    }

    /// Does the table already have an entry for this CRI anywhere within it
    #[must_use]
    fn contains_cri(&self, cri: &Rc<Cri>) -> bool {
        self.columns.iter().any(|column| column.contains_cri(cri))
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
    pub fn get_row(&self, row: Row) -> Vec<Option<Rc<Cri>>> {
        let mut row_data = vec![None; self.columns.len()];

        row_data
            .iter_mut()
            .zip(self.columns.iter())
            .for_each(|(item, column)| {
                *item = column.get_row(row);
            });

        row_data
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

    fn create_cri_for_rows(name: &str, rows: &[Row]) -> Rc<Cri> {
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
        Rc::new(cri)
    }

    #[test]
    fn test_all_the_things() {
        // This should populate the first column fully
        let cri1 = create_cri_for_rows(
            "cri1",
            &[Row::Strength, Row::IdentityFraud, Row::ActivityHistory],
        );
        let cri2 = create_cri_for_rows("cri2", &[Row::Validity]);
        let cri3 = create_cri_for_rows("cri3", &[Row::Verification]);
        let cri4 = create_cri_for_rows("cri4", &[Row::Other]);

        // This should appear on column 2
        let cri5 = create_cri_for_rows("cri5", &[Row::Strength, Row::IdentityFraud]);

        // This should appear on column 3
        let cri6 = create_cri_for_rows("cri6", &[Row::IdentityFraud]);

        let mut cris = vec![
            cri1.clone(),
            cri2.clone(),
            cri3.clone(),
            cri4.clone(),
            cri5.clone(),
            cri6.clone(),
        ];
        cris.shuffle(&mut rand::rng());

        let table = Table::new(cris);

        let strength_row: Vec<_> = table
            .get_row(Row::Strength)
            .into_iter()
            .map(|cri| cri.map(|cri| cri.name.clone()))
            .collect();
        assert_eq!(
            strength_row,
            vec![Some("cri1".to_string()), Some("cri5".to_string()), None]
        );
    }
}
