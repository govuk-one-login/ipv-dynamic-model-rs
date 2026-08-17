use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum StrengthScore {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum ValidityScore {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum ActivityHistoryScore {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum IdentityFraudScore {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum VerificationScore {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[cfg(feature = "test-utils")]
mod tests_utils {
    use super::*;
    use crate::test_utils::RandomChoice;
    use rand::prelude::*;

    impl RandomChoice for StrengthScore {
        fn random_choice() -> Self {
            let choices = [
                StrengthScore::One,
                StrengthScore::Two,
                StrengthScore::Three,
                StrengthScore::Four,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }

    impl RandomChoice for ValidityScore {
        fn random_choice() -> Self {
            let choices = [
                ValidityScore::One,
                ValidityScore::Two,
                ValidityScore::Three,
                ValidityScore::Four,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }

    impl RandomChoice for ActivityHistoryScore {
        fn random_choice() -> Self {
            let choices = [
                ActivityHistoryScore::One,
                ActivityHistoryScore::Two,
                ActivityHistoryScore::Three,
                ActivityHistoryScore::Four,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }

    impl RandomChoice for IdentityFraudScore {
        fn random_choice() -> Self {
            let choices = [
                IdentityFraudScore::Zero,
                IdentityFraudScore::One,
                IdentityFraudScore::Two,
                IdentityFraudScore::Three,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }

    impl RandomChoice for VerificationScore {
        fn random_choice() -> Self {
            let choices = [
                VerificationScore::One,
                VerificationScore::Two,
                VerificationScore::Three,
                VerificationScore::Four,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }
}
