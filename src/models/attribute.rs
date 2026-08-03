use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
pub mod tests_utils {
    use super::*;
    use crate::test_utils::random_string;

    pub fn create_test_attribute() -> Attribute {
        Attribute {
            name: random_string("name"),
            description: random_string("description"),
        }
    }
}
