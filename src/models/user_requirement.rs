use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[should_panic] // This test will fail until yaml_serde fixes nested enum maps
    fn test_nested_serialize_deserialize() {
        let user_requirement = Wrapper(UserRequirement::All(vec![
            UserRequirement::UkPassport,
            UserRequirement::Any(vec![
                UserRequirement::UkDrivingLicense,
                UserRequirement::BankAccount,
            ]),
        ]));
        let serialized = yaml_serde::to_string(&user_requirement).unwrap();
        assert_eq!(
            serialized,
            "all:\n- UK Passport\n- any:\n  - UK Driving License\n  - Bank Account\n"
        );
        let deserialized: Wrapper = yaml_serde::from_str(&serialized).unwrap();
        assert_eq!(user_requirement.0, deserialized.0);
    }
}
