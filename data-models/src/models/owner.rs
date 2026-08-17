use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Owner {
    Kiwi,
    Lime,
    Orange,
    Other,
    Mobile,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kiwi => write!(f, "Kiwi"),
            Self::Lime => write!(f, "Lime"),
            Self::Orange => write!(f, "Orange"),
            Self::Mobile => write!(f, "Mobile"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[cfg(feature = "test-utils")]
mod tests_utils {
    use super::*;
    use crate::test_utils::RandomChoice;
    use rand::prelude::*;
    impl RandomChoice for Owner {
        fn random_choice() -> Self {
            let choices = [Self::Kiwi, Self::Lime, Self::Orange, Self::Other];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }
}
