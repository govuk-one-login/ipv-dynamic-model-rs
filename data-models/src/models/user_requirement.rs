use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum UserRequirement {
    #[serde(rename = "UK Passport")]
    UkPassport,
    #[serde(rename = "ICAO9303 Machine Readable Travel Document")]
    ICAO9303,
    #[serde(rename = "International Passport")]
    InternationalPassport,
    #[serde(rename = "UK Driving License")]
    UkDrivingLicense,
    #[serde(rename = "Bank Account")]
    BankAccount,
    #[serde(rename = "BRP Document")]
    BRPDocument,
    #[serde(rename = "National Insurance Number")]
    NationalInsuranceNumber,
    #[serde(rename = "Credit History")]
    CreditHistory,
    #[serde(rename = "Benefits History")]
    BenefitsHistory,
    #[serde(rename = "Smart Phone")]
    SmartPhone,
    #[serde(rename = "all")]
    All(Vec<Self>),
    #[serde(rename = "any")]
    Any(Vec<Self>),
}

impl UserRequirement {
    #[must_use]
    pub const fn all_valid_individual_requirements() -> [Self; 10] {
        [
            Self::UkPassport,
            Self::ICAO9303,
            Self::InternationalPassport,
            Self::UkDrivingLicense,
            Self::BankAccount,
            Self::BRPDocument,
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
    use crate::test_utils::{RandomChoice, random_vec};
    use rand::prelude::IndexedRandom;

    impl RandomChoice for UserRequirement {
        fn random_choice() -> Self {
            let non_recursive_choices = [
                Self::UkPassport,
                Self::ICAO9303,
                Self::InternationalPassport,
                Self::UkDrivingLicense,
                Self::BankAccount,
                Self::BRPDocument,
                Self::NationalInsuranceNumber,
                Self::CreditHistory,
                Self::BenefitsHistory,
                Self::SmartPhone,
            ];
            let random_non_recursive = || {
                non_recursive_choices
                    .choose(&mut rand::rng())
                    .unwrap()
                    .clone()
            };

            let recursive_choices = [
                Self::UkPassport,
                Self::ICAO9303,
                Self::InternationalPassport,
                Self::UkDrivingLicense,
                Self::BankAccount,
                Self::BRPDocument,
                Self::NationalInsuranceNumber,
                Self::CreditHistory,
                Self::BenefitsHistory,
                Self::SmartPhone,
                Self::All(random_vec(2, 3, random_non_recursive)),
                Self::Any(random_vec(2, 3, random_non_recursive)),
            ];
            recursive_choices.choose(&mut rand::rng()).unwrap().clone()
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
        let user_requirement = Wrapper(UserRequirement::UkPassport);
        let serialized = yaml_serde::to_string(&user_requirement).unwrap();
        assert_eq!(serialized, "UK Passport\n");
        let deserialized: Wrapper = yaml_serde::from_str(&serialized).unwrap();
        assert_eq!(user_requirement.0, deserialized.0);
    }

    #[test]
    fn test_nested_serialize_deserialize() {
        let user_requirement = Wrapper(UserRequirement::All(vec![
            UserRequirement::UkPassport,
            UserRequirement::Any(vec![
                UserRequirement::UkDrivingLicense,
                UserRequirement::BankAccount,
            ]),
        ]));
        let serialized = yaml_serde::to_string(&user_requirement).unwrap();
        // ToDo: Fix this test when yaml_serde fixes nested enum maps
        assert_ne!(
            serialized,
            "all:\n- UK Passport\n- any:\n  - UK Driving License\n  - Bank Account\n"
        );
        // let deserialized: Wrapper = yaml_serde::from_str(&serialized).unwrap();
        // assert_eq!(user_requirement.0, deserialized.0);
    }
}
