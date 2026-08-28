use std::fmt;

/// A simplified report on a system's status.
///
/// - If the system's up and not overloaded it's [`ServiceStatus::Good`]
/// - If the system's up but at or over capacity it's considered [`ServiceStatus::Degraded`]
/// - If the system's down you'll get [`ServiceStatus::Off`]
///
/// Note, two degraded systems may not be exactly equivalent, so the type implements [`PartialEq`]
/// but not [`Eq`]
#[allow(
    clippy::derive_partial_eq_without_eq,
    reason = "Two degraded systems are not be exactly equivalent"
)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ServiceStatus {
    Good,
    Degraded,
    Off,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Good => write!(f, "good"),
            Self::Degraded => write!(f, "degraded"),
            Self::Off => write!(f, "off"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ServiceStatus::Good), "good");
        assert_eq!(format!("{}", ServiceStatus::Degraded), "degraded");
        assert_eq!(format!("{}", ServiceStatus::Off), "off");
    }
}
