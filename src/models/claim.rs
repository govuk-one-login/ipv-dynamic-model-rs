use crate::models::attribute::Attribute;
use crate::models::scores::{HasScores, Scores};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claim {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(flatten)]
    pub scores: Scores,
}

impl HasScores for Claim {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

#[cfg(test)]
pub mod tests_utils {
    use super::*;
    use crate::test_utils::*;

    impl CreateTestSubject for Claim {
        fn create_test_subject() -> Self {
            Self {
                name: random_string("name"),
                description: random_string("description"),
                attributes: random_vec(0, 5, Attribute::create_test_subject),
                scores: Scores::create_test_subject(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::CreateTestSubject;
    use super::*;

    #[test]
    fn test_has_scores() {
        let claim = Claim::create_test_subject();
        assert_eq!(claim.scores(), &claim.scores);
    }
}
