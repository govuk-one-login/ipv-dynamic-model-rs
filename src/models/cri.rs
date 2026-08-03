use serde::{Deserialize, Serialize};
use crate::models::claim::Claim;
use crate::models::score::{ActivityHistoryScore, IdentityFraudScore, StrengthScore, ValidityScore, VerificationScore};
use crate::models::user_requirement::UserRequirement;

type RequestsPerSecond = f64;
type SuccessRate = f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cri {
    pub name: String,
    pub description: String,
    pub throughput: RequestsPerSecond,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub possible_cis: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub mitigates_cis: Vec<String>,
    pub success_rate: SuccessRate,
    #[serde(with = "yaml_serde::with::singleton_map")]
    pub user_requirements: Option<UserRequirement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub claims_required: Vec<Claim>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub claims_produced: Vec<Claim>,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub comments: String,
    pub strength_score: Option<StrengthScore>,
    pub validity_score: Option<ValidityScore>,
    pub identity_fraud_score: Option<IdentityFraudScore>,
    pub activity_history_score: Option<ActivityHistoryScore>,
    pub verification_score: Option<VerificationScore>,
}
