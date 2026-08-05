use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
pub mod tests_utils {
    use super::*;
    use crate::test_utils::{CreateTestSubject, random_string};

    impl CreateTestSubject for Attribute {
        fn create_test_subject() -> Self {
            Self {
                name: random_string("name"),
                description: random_string("description"),
            }
        }
    }
}
