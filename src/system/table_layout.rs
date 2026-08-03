use crate::models::cri::Cri;
use std::rc::Rc;

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
    #[must_use]
    pub const fn cri_appears_in_row(&self, cri: &Cri) -> bool {
        match self {
            Self::Strength => cri.strength_score.is_some(),
            Self::Validity => cri.validity_score.is_some(),
            Self::IdentityFraud => cri.identity_fraud_score.is_some(),
            Self::ActivityHistory => cri.activity_history_score.is_some(),
            Self::Verification => cri.verification_score.is_some(),
            Self::Other => !cri.has_score(),
        }
    }
}

const ROW_ORDER: [Row; 6] = [
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
        if !cri.has_score() {
            self.other = Some(cri.clone());
            return;
        }
        if cri.strength_score.is_some() {
            self.strength = Some(cri.clone());
        }
        if cri.validity_score.is_some() {
            self.validity = Some(cri.clone());
        }
        if cri.identity_fraud_score.is_some() {
            self.identity_fraud = Some(cri.clone());
        }
        if cri.activity_history_score.is_some() {
            self.activity_history = Some(cri.clone());
        }
        if cri.verification_score.is_some() {
            self.verification = Some(cri.clone());
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    #[must_use]
    pub fn new(mut cris: Vec<Rc<Cri>>) -> Self {
        // Sort the CRIs by number of scores they contain
        cris.sort_by_key(|cri| cri.score_count());

        // We'll prevent reallocation by allocating for worst case scenario
        let mut table = Self {
            columns: Vec::with_capacity(cris.len()),
        };

        for row in ROW_ORDER {
            for cri in &cris {
                if table.contains_cri(cri) || row.cri_appears_in_row(cri) {
                    continue;
                }
                table.get_first_available_column(row).add_cri(cri);
            }
        }

        table
    }

    #[must_use]
    fn contains_cri(&self, cri: &Rc<Cri>) -> bool {
        self.columns.iter().any(|column| column.contains_cri(cri))
    }

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
            .unwrap()
    }
}
