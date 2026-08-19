use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Owner {
    Kiwi,
    Lime,
    Orange,
    External,
    Mobile,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kiwi => write!(f, "kiwi"),
            Self::Lime => write!(f, "lime"),
            Self::Orange => write!(f, "orange"),
            Self::Mobile => write!(f, "mobile"),
            Self::External => write!(f, "external"),
        }
    }
}

#[cfg(feature = "test-utils")]
mod test_utils {
    use super::*;
    use crate::test_utils::RandomChoice;
    use rand::prelude::*;
    impl RandomChoice for Owner {
        fn random_choice() -> Self {
            let choices = [Self::Kiwi, Self::Lime, Self::Orange, Self::External];
            *choices.choose(&mut rand::rng()).unwrap() // Safe as slice is not empty
        }
    }
}
