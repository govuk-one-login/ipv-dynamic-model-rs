use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ScoreType {
    Strength,
    Validity,
    Verification,
    ActivityHistory,
    IdentityFraud,
}

impl ScoreType {
    /// Gives a list of which score we think its most important for a user to get first.
    ///
    /// The first entry is most important, and it continues in decreasing order
    ///
    /// Using an array was chosen over repr as we want to be able to change the values
    #[must_use]
    pub const fn order_of_score_importance() -> [Self; 5] {
        [
            Self::Strength,
            Self::Validity,
            Self::Verification,
            Self::ActivityHistory,
            Self::IdentityFraud,
        ]
    }

    #[allow(
        clippy::missing_panics_doc,
        reason = "All scores types appear in order_of_score_importance(), so panic never happens"
    )]
    #[must_use]
    pub fn compare_importance_with(self, other: Self) -> Ordering {
        let order = Self::order_of_score_importance();

        let self_pos = order.iter().rev().position(|t| self == *t).unwrap(); // Safe because all cases are in the list
        let other_pos = order.iter().rev().position(|t| other == *t).unwrap(); // Safe because all cases are in the list

        self_pos.cmp(&other_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_importance() {
        assert_eq!(
            ScoreType::Strength.compare_importance_with(ScoreType::ActivityHistory),
            Ordering::Greater
        );
    }
}
