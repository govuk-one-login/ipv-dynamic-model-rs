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

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    use crate::test_utils::{CreateTestSubject, random_string, random_vec};

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
