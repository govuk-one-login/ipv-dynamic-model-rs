use crate::models::claim::Claim;
use crate::models::score::{
    ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore, VerificationScore,
};
use crate::models::user_requirement::UserRequirement;
use serde::{Deserialize, Serialize};

type RequestsPerSecond = f64;
type SuccessRate = f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cri {
    pub name: String,
    pub description: String,
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
    pub strength_score: Option<StrengthScore>,
    pub validity_score: Option<ValidityScore>,
    pub identity_fraud_score: Option<IdentityFraudScore>,
    pub activity_history_score: Option<ActivityHistoryScore>,
    pub verification_score: Option<VerificationScore>,
}

impl Cri {
    /// Does this CRI produce any type of score
    #[must_use]
    pub const fn has_score(&self) -> bool {
        self.strength_score.is_some()
            || self.validity_score.is_some()
            || self.identity_fraud_score.is_some()
            || self.activity_history_score.is_some()
            || self.verification_score.is_some()
    }

    /// How many types of scores does this CRI produce?
    #[must_use]
    pub fn score_count(&self) -> usize {
        [
            self.strength_score.is_some(),
            self.validity_score.is_some(),
            self.identity_fraud_score.is_some(),
            self.activity_history_score.is_some(),
            self.verification_score.is_some(),
        ]
        .iter()
        .copied()
        .filter(|score| *score)
        .count()
    }
}

impl PartialEq for Cri {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
