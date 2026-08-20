use crate::prelude::Claim;

pub mod identity_profile;
pub mod known_profiles;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Confidence {
    P1,
    P2,
    P3,
    P4,
}

#[derive(Default, Debug, Clone)]
pub struct Identity {
    claims: Vec<Claim>,
}

impl Identity {
    pub fn add_claim(&mut self, claim: Claim) {
        self.claims.push(claim);
    }
}
