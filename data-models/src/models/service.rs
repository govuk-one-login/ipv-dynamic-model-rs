use crate::prelude::*;
use core::fmt;
use core::ops::Deref;

// Two degraded systems are not equivalent
#[allow(clippy::derive_partial_eq_without_eq)]
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

/// Service wraps a CRI model to make a pretend running service of that model. You can turn the
/// service on/off, simulate traffic, etc.
#[derive(Debug, Clone)]
pub struct Service {
    pub cri: Cri,
    pub active: bool,
    pub traffic: RequestsPerSecond,
}

impl Service {
    #[must_use]
    pub fn new(cri: Cri) -> Self {
        Self {
            cri,
            active: true,
            traffic: RequestsPerSecond::default(),
        }
    }

    /// Lets you turn the service on of off
    #[must_use]
    pub const fn get_active(&self) -> bool {
        self.active
    }

    /// Turn the service on
    pub const fn turn_on(&mut self) {
        self.active = true;
    }

    /// Turn the service off
    pub const fn turn_off(&mut self) {
        self.active = false;
    }

    /// Set the amount of traffic flowing to the service, returns the remaining capacity
    ///
    /// Note: remaining capacity is always a non-zero number so if traffic is higher than the CRI's
    /// potential throughput, this will return 0.0;
    pub fn set_traffic(&mut self, traffic: RequestsPerSecond) -> RequestsPerSecond {
        self.traffic = traffic;
        self.cri.throughput.remaining_capacity(self.traffic)
    }

    /// Tells you the Service Status.
    ///
    /// - If the service is off, you will get `ServiceStatus::Off`
    /// - If the traffic is at or exceeding capacity you will get `ServiceStatus::Degraded`
    /// - Otherwise we assume the service is ok, and you get `ServiceStatus::Good`
    #[must_use]
    pub fn get_status(&self) -> ServiceStatus {
        if !self.active {
            return ServiceStatus::Off;
        }

        if self.remaining_capacity() < 1.0 {
            return ServiceStatus::Degraded;
        }

        ServiceStatus::Good
    }

    /// Tells you remaining capacity on the service. If the number is less than `1.0`, the service
    /// is considered saturated. This number is never negative.
    #[must_use]
    fn remaining_capacity(&self) -> f64 {
        self.cri
            .throughput
            .remaining_capacity(self.traffic)
            .as_f64()
    }
}

impl Deref for Service {
    type Target = Cri;

    fn deref(&self) -> &Self::Target {
        &self.cri
    }
}

impl PartialEq for Service {
    fn eq(&self, other: &Self) -> bool {
        self.cri == other.cri
    }
}

impl PartialEq<Cri> for Service {
    fn eq(&self, other: &Cri) -> bool {
        other == &self.cri
    }
}

impl From<Cri> for Service {
    fn from(cri: Cri) -> Self {
        Self {
            cri,
            active: true,
            traffic: RequestsPerSecond::default(),
        }
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    use crate::test_utils::CreateTestSubject;

    impl CreateTestSubject for Service {
        fn create_test_subject() -> Self {
            Self {
                cri: Cri::create_test_subject(),
                active: true,
                traffic: RequestsPerSecond::create_test_subject(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::CreateTestSubject;

    #[test]
    fn test_new() {
        let cri = Cri::create_test_subject();
        let service = Service::new(cri.clone());

        // The service should contain the CRI, start activated and have no traffic
        assert_eq!(service.cri, cri);
        assert_eq!(service.active, true);
        assert_eq!(service.traffic, RequestsPerSecond::new(0.0).unwrap());
    }

    #[test]
    fn test_set_active() {
        let cri = Cri::create_test_subject();
        let mut service = Service::new(cri.clone());

        assert_eq!(service.get_active(), true);
        service.turn_off();
        assert_eq!(service.get_active(), false);
        service.turn_on();
        assert_eq!(service.get_active(), true);
    }

    #[test]
    fn test_set_traffic() {
        let cri = Cri::create_test_subject();
        let mut service = Service::new(cri.clone());

        assert_eq!(*service.traffic, 0.0);
        service.set_traffic(RequestsPerSecond::new(10.0).unwrap());
        assert_eq!(*service.traffic, 10.0);
    }

    #[test]
    fn test_get_status() {
        let mut cri = Cri::create_test_subject();
        cri.throughput = RequestsPerSecond::new(10.0).unwrap();
        let mut service = Service::new(cri.clone());

        assert_eq!(service.get_status(), ServiceStatus::Good);

        service.set_traffic(RequestsPerSecond::new(1000.0).unwrap());
        assert_eq!(service.get_status(), ServiceStatus::Degraded);

        service.turn_off();
        assert_eq!(service.get_status(), ServiceStatus::Off);
    }
}
