use crate::models::scores::Scores;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct IdentityProfile(pub Scores, pub Option<Scores>, pub Option<Scores>);
