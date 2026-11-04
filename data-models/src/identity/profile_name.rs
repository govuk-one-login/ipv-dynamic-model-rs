use super::known_profile::*;
use core::fmt;
use std::str::FromStr;
use thiserror::Error;

/// This macro removes a lot of repetition. It creates an enum called `IdentityProfileName` that
/// contains the name of every identity profile, and adds a way to go from that enum to the profile
/// data itself as well as convert the name to and from a string, and all the tests.
macro_rules! identity_profile_names {
    ($($profile:ident),+) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum IdentityProfileName {
            $($profile),+
        }

        impl fmt::Display for IdentityProfileName {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$profile => write!(f, stringify!($profile)),)+
                }
            }
        }


        #[derive(Error, Debug)]
        pub enum IdentityProfileNameFromStrError {
            #[error("Unknown Identity Profile Name")]
            UnknownIdentityProfileName,
        }

        impl FromStr for IdentityProfileName {
            type Err = IdentityProfileNameFromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($profile) => Ok(Self::$profile),)+
                    _ => Err(IdentityProfileNameFromStrError::UnknownIdentityProfileName)
                }

            }
        }

        impl From<IdentityProfileName> for KnownIdentityProfile {
            fn from(value: IdentityProfileName) -> Self {
                match value {
                    $(IdentityProfileName::$profile => $profile,)+
                }
            }
        }


        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_profile_from_name() {
                // Check that we can get the profile from its name (written backwards for type hint)
                $( assert_eq!($profile, IdentityProfileName::$profile.into()); )+
            }

            #[test]
            fn test_name_display() {
                // Check the name prints out correctly
                $( assert_eq!(format!("{}", IdentityProfileName::$profile), stringify!($profile)); )+
            }
        }


    }
}

identity_profile_names! {
    // Very High
    V1A,
    V1B,
    V1C,
    V1D,
    V2A,
    V2B,
    V2C,
    V2D,
    V3A,
    // High
    H1A,
    H1B,
    H1C,
    H2A,
    H2B,
    H2C,
    H2D,
    H2E,
    H3A,
    // Medium
    M1A,
    M1B,
    M1C,
    M1D,
    M2A,
    M2B,
    M2C,
    M3A,
    // Low
    L1A,
    L1B,
    L1C,
    L2A,
    L2B,
    L3A
}
