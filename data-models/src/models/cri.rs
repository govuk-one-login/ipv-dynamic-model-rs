use crate::models::claim::Claim;
use crate::models::owner::Owner;
use crate::models::requests_per_second::RequestsPerSecond;
use crate::models::scores::{HasScores, Scores};
use crate::models::user_requirement::UserRequirement;
use serde::{Deserialize, Serialize};

type SuccessRate = f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cri {
    pub name: String,
    pub description: String,
    pub owner: Owner,
    pub throughput: RequestsPerSecond,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub possible_cis: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigates_cis: Vec<String>,
    pub success_rate: SuccessRate,
    #[serde(with = "yaml_serde::with::singleton_map")]
    pub user_requirements: Option<UserRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims_required: Vec<Claim>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims_produced: Vec<Claim>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub comments: String,
    #[serde(flatten)]
    pub scores: Scores,
}

impl Cri {
    #[must_use]
    pub fn can_mitigate_ci(&self, ci: &String) -> bool {
        self.mitigates_cis.contains(ci)
    }

    #[must_use]
    pub fn can_mitigate_any_of_ci(&self, cis: &[&String]) -> bool {
        cis.iter().any(|ci| self.can_mitigate_ci(ci))
    }
}

impl PartialEq for Cri {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl HasScores for Cri {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    use crate::models::owner::Owner;
    use crate::test_utils::{CreateTestSubject, RandomChoice, random_string, random_vec};
    use rand::random_range;

    impl CreateTestSubject for Cri {
        fn create_test_subject() -> Self {
            Self {
                name: random_string("name"),
                description: random_string("description"),
                owner: Owner::random_choice(),
                throughput: RequestsPerSecond::create_test_subject(),
                possible_cis: vec![],
                mitigates_cis: vec![],
                success_rate: random_range(0.0..1.0),
                user_requirements: UserRequirement::random_choice_option(0.3),
                claims_required: random_vec(0, 3, Claim::create_test_subject),
                claims_produced: random_vec(0, 3, Claim::create_test_subject),
                comments: random_string("description"),
                scores: Scores::create_test_subject(),
            }
        }
    }
}
