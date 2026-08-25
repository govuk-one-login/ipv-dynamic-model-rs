use crate::prelude::Claim;

pub mod identity_profile;
pub mod known_profiles;

/// Proofing is from the [Vectors of Trust RFC](https://datatracker.ietf.org/doc/html/rfc8485#section-2.1)
/// which for us maps to our Identity Profile levels (Low, Medium, High, Very High)
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Proofing {
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
