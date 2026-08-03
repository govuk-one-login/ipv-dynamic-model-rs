use serde::{Deserialize, Serialize};
use crate::models::attribute::Attribute;
use crate::models::score::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claim {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    pub strength_score: Option<StrengthScore>,
    pub validity_score: Option<ValidityScore>,
    pub identity_fraud_score: Option<IdentityFraudScore>,
    pub activity_history_score: Option<ActivityHistoryScore>,
    pub verification_score: Option<VerificationScore>,
}

#[cfg(test)]
pub mod tests_utils {
    use super::*;
    use crate::models::attribute::tests_utils::create_test_attribute;
    use crate::test_utils::*;

    pub fn create_test_claim() -> Claim {
        Claim {
            name: random_string("name"),
            description: random_string("description"),
            attributes: random_vec(0, 5, create_test_attribute),
            strength_score: StrengthScore::random_choice_option(0.3),
            validity_score: ValidityScore::random_choice_option(0.3),
            identity_fraud_score: IdentityFraudScore::random_choice_option(0.3),
            activity_history_score: ActivityHistoryScore::random_choice_option(0.3),
            verification_score: VerificationScore::random_choice_option(0.3),
        }
    }
}
