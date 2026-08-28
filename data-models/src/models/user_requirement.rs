use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserRequirement {
    #[serde(rename = "Passport")]
    Passport,
    #[serde(rename = "Driving License")]
    DrivingLicense,
    #[serde(rename = "Bank Account")]
    BankAccount,
    #[serde(rename = "National Insurance Number")]
    NationalInsuranceNumber,
    #[serde(rename = "Credit History")]
    CreditHistory,
    #[serde(rename = "Benefits History")]
    BenefitsHistory,
    #[serde(rename = "Smart Phone")]
    SmartPhone,
}

impl UserRequirement {
    #[must_use]
    pub const fn all_valid_individual_requirements() -> [Self; 7] {
        [
            Self::Passport,
            Self::DrivingLicense,
            Self::BankAccount,
            Self::NationalInsuranceNumber,
            Self::CreditHistory,
            Self::BenefitsHistory,
            Self::SmartPhone,
        ]
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    use crate::test_utils::RandomChoice;
    use rand::prelude::IndexedRandom;

    impl RandomChoice for UserRequirement {
        fn random_choice() -> Self {
            let choices = [
                Self::Passport,
                Self::DrivingLicense,
                Self::BankAccount,
                Self::NationalInsuranceNumber,
                Self::CreditHistory,
                Self::BenefitsHistory,
                Self::SmartPhone,
            ];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Have to use a stupid wrapper to make this work
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Wrapper(#[serde(with = "yaml_serde::with::singleton_map")] pub UserRequirement);

    #[test]
    fn test_simple_serialize_deserialize() {
        let user_requirement = Wrapper(UserRequirement::Passport);
        let serialized = yaml_serde::to_string(&user_requirement).unwrap();
        assert_eq!(serialized, "Passport\n");
        let deserialized: Wrapper = yaml_serde::from_str(&serialized).unwrap();
        assert_eq!(user_requirement.0, deserialized.0);
    }
}
