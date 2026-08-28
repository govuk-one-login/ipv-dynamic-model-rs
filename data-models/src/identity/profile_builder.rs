use crate::prelude::*;
use std::cmp::Ordering;
use crate::identity::Proofing;

#[derive(Default, Debug, Copy, Clone)]
pub struct ProfileBuilder {
    strength: [Option<StrengthScore>; 3],
    validity: [Option<ValidityScore>; 3],
    activity_history: Option<ActivityHistoryScore>,
    identity_fraud: Option<IdentityFraudScore>,
    verification: Option<VerificationScore>,
}

impl ProfileBuilder {
    #[must_use]
    pub fn is_at_least_as_strong_as(&self, other: &KnownIdentityProfile) -> bool {
        self.strength[0] >= other.1.strength
            && self.strength[1] >= other.2.and_then(|o| o.strength)
            && self.strength[2] >= other.3.and_then(|o| o.strength)
            && self.validity[0] >= other.1.validity
            && self.validity[1] >= other.2.and_then(|o| o.validity)
            && self.validity[2] >= other.3.and_then(|o| o.validity)
            && self.activity_history >= other.1.activity_history
            && self.identity_fraud >= other.1.identity_fraud
            && self.verification >= other.1.verification
    }

    #[must_use]
    pub fn to_known_profile(&self, minimum: Proofing) -> Option<KnownIdentityProfile> {
        let possible_profiles = KnownIdentityProfile::profiles_of(minimum);
        possible_profiles.iter().find(|profile| self >= profile).copied()
    }
}

impl PartialEq<KnownIdentityProfile> for ProfileBuilder {
    fn eq(&self, other: &KnownIdentityProfile) -> bool {
        self.strength[0] == other.1.strength
            && self.strength[1] == other.2.and_then(|o| o.strength)
            && self.strength[2] == other.3.and_then(|o| o.strength)
            && self.validity[0] == other.1.validity
            && self.validity[1] == other.2.and_then(|o| o.validity)
            && self.validity[2] == other.3.and_then(|o| o.validity)
            && self.activity_history == other.1.activity_history
            && self.identity_fraud == other.1.identity_fraud
            && self.verification == other.1.verification
    }
}

impl PartialOrd<KnownIdentityProfile> for ProfileBuilder {
    fn partial_cmp(&self, other: &KnownIdentityProfile) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.is_at_least_as_strong_as(other) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_known_profile() {
        let bigger_than_known = ProfileBuilder {
            strength: [Some(StrengthScore::Four), None, None],
            validity: [Some(ValidityScore::Four), None, None],
            activity_history: None,
            identity_fraud: None,
            verification: Some(VerificationScore::Three),
        };
        let profile = bigger_than_known.to_known_profile(Proofing::P1);
        assert_eq!(profile, Some(M1C));

        let equal_to_known = ProfileBuilder {
            strength: [Some(StrengthScore::Three), None, None],
            validity: [Some(ValidityScore::Three), None, None],
            activity_history: None,
            identity_fraud: None,
            verification: Some(VerificationScore::Three),
        };
        let profile = equal_to_known.to_known_profile(Proofing::P1);
        assert_eq!(profile, Some(M1C));

        // Does not match higher proofing
        let profile = equal_to_known.to_known_profile(Proofing::P3);
        assert_eq!(profile, None);
    }
}
