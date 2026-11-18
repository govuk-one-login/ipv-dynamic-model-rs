use crate::identity::Proofing;
use crate::prelude::*;
use std::cmp::Ordering;

#[derive(Default, Debug, Copy, Clone)]
pub struct ProfileBuilder {
    strength: [Option<StrengthScore>; 3],
    validity: [Option<ValidityScore>; 3],
    activity_history: Option<ActivityHistoryScore>,
    identity_fraud: Option<IdentityFraudScore>,
    verification: Option<VerificationScore>,
}

impl ProfileBuilder {
    /// Create an empty [`ProfileBuilder`]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

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
        possible_profiles
            .iter()
            .find(|profile| self >= profile)
            .copied()
    }

    /// Adds the [`StrengthScore`] to the profile. If there is already three strength scores set
    /// against the profile, then the given score will replace the lowest score if it is higher than
    /// that one and, either way, the lowest of those two values will be returned.
    #[must_use]
    pub fn set_strength(&mut self, strength: StrengthScore) -> Option<StrengthScore> {
        if let Some(current_strength) = self.strength[2]
            && current_strength > strength
        {
            Some(strength)
        } else {
            let old = self.strength[2].replace(strength);
            // Note: sort then reverse is just as fast as writing a sort function
            self.strength.sort();
            self.strength.reverse();
            old
        }
    }

    /// Adds the [`ValidityScore`] to the profile. If there is already three validity scores set
    /// against the profile, then the given score will replace the lowest score if it is higher than
    /// that one and, either way, the lowest of those two values will be returned.
    #[must_use]
    pub fn set_validity(&mut self, validity: ValidityScore) -> Option<ValidityScore> {
        if let Some(current_validity) = self.validity[2]
            && current_validity > validity
        {
            Some(validity)
        } else {
            let old = self.validity[2].replace(validity);
            // Note: sort then reverse is just as fast as writing a sort function
            self.validity.sort();
            self.validity.reverse();
            old
        }
    }

    /// Sets the [`ActivityHistoryScore`] for the profile. If there is already a score then the
    /// higher score is set, and the lower score is returned.
    #[must_use]
    pub fn set_activity_history(
        &mut self,
        activity_history: ActivityHistoryScore,
    ) -> Option<ActivityHistoryScore> {
        if let Some(current_history) = self.activity_history
            && current_history > activity_history
        {
            Some(activity_history)
        } else {
            self.activity_history.replace(activity_history)
        }
    }

    /// Sets the [`IdentityFraudScore`] for the profile. If there is already a score then the
    /// higher score is set, and the lower score is returned.
    #[must_use]
    pub fn set_identity_fraud(
        &mut self,
        identity_fraud: IdentityFraudScore,
    ) -> Option<IdentityFraudScore> {
        if let Some(current_fraud) = self.identity_fraud
            && current_fraud > identity_fraud
        {
            Some(identity_fraud)
        } else {
            self.identity_fraud.replace(identity_fraud)
        }
    }

    /// Sets the [`VerificationScore`] for the profile. If there is already a score then the
    /// higher score is set, and the lower score is returned.
    #[must_use]
    pub fn set_verification(
        &mut self,
        verification: VerificationScore,
    ) -> Option<VerificationScore> {
        if let Some(current_verification) = self.verification
            && current_verification > verification
        {
            Some(verification)
        } else {
            self.verification.replace(verification)
        }
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

    #[test]
    fn test_set_activity_history() {
        let mut builder = ProfileBuilder::new();
        // There is no score yet so nothing to return
        assert_eq!(
            builder.set_activity_history(ActivityHistoryScore::Two),
            None
        );
        // This won't change the score, so the provided One is returned
        assert_eq!(
            builder.set_activity_history(ActivityHistoryScore::One),
            Some(ActivityHistoryScore::One)
        );
        // This will, so the currently stored Two is returned
        assert_eq!(
            builder.set_activity_history(ActivityHistoryScore::Three),
            Some(ActivityHistoryScore::Two)
        );
    }

    #[test]
    fn test_set_identity_fraud() {
        let mut builder = ProfileBuilder::new();
        // There is no score yet so nothing to return
        assert_eq!(builder.set_identity_fraud(IdentityFraudScore::Two), None);
        // This won't change the score, so the provided One is returned
        assert_eq!(
            builder.set_identity_fraud(IdentityFraudScore::One),
            Some(IdentityFraudScore::One)
        );
        // This will, so the currently stored Two is returned
        assert_eq!(
            builder.set_identity_fraud(IdentityFraudScore::Three),
            Some(IdentityFraudScore::Two)
        );
    }

    #[test]
    fn test_set_verification() {
        let mut builder = ProfileBuilder::new();
        // There is no score yet so nothing to return
        assert_eq!(builder.set_verification(VerificationScore::Two), None);
        // This won't change the score, so the provided One is returned
        assert_eq!(
            builder.set_verification(VerificationScore::One),
            Some(VerificationScore::One)
        );
        // This will, so the currently stored Two is returned
        assert_eq!(
            builder.set_verification(VerificationScore::Three),
            Some(VerificationScore::Two)
        );
    }

    #[test]
    fn test_set_strength() {
        let mut builder = ProfileBuilder::new();
        // The first 3 scores will always return None
        assert_eq!(builder.set_strength(StrengthScore::Two), None);
        assert_eq!(builder.set_strength(StrengthScore::One), None);
        assert_eq!(builder.set_strength(StrengthScore::Three), None);

        assert_eq!(builder.strength, [
            Some(StrengthScore::Three),
            Some(StrengthScore::Two),
            Some(StrengthScore::One),
        ]);
        // This time the final number will be popped
        assert_eq!(
            builder.set_strength(StrengthScore::Three),
            Some(StrengthScore::One)
        );

        assert_eq!(builder.strength, [
            Some(StrengthScore::Three),
            Some(StrengthScore::Three),
            Some(StrengthScore::Two),
        ]);
    }

    #[test]
    fn test_set_validity() {
        let mut builder = ProfileBuilder::new();
        // The first 3 scores will always return None
        assert_eq!(builder.set_validity(ValidityScore::Two), None);
        assert_eq!(builder.set_validity(ValidityScore::One), None);
        assert_eq!(builder.set_validity(ValidityScore::Three), None);

        assert_eq!(builder.validity, [
            Some(ValidityScore::Three),
            Some(ValidityScore::Two),
            Some(ValidityScore::One),
        ]);
        // This time the final number will be popped
        assert_eq!(
            builder.set_validity(ValidityScore::Three),
            Some(ValidityScore::One)
        );

        assert_eq!(builder.validity, [
            Some(ValidityScore::Three),
            Some(ValidityScore::Three),
            Some(ValidityScore::Two),
        ]);
    }
}
