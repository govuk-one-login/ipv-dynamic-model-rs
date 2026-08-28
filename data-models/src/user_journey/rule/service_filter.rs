//! # An important not for these filters.
//!
//! In order to prevent copying of the [`Service`] data, they must take a reference to a reference.
//!
//! For this reason all filters have the type `impl Fn(&&Service) -> bool`

use crate::prelude::Service;
use crate::user_journey::journey::Journey;

/// Generic type for a function that takes a [`Vec`] of [`Service`]s and filters out useless ones
///
/// As we step through a user [`Journey`] and visit different services, not all services will be
/// appropriate to visit. [`ServiceFilter`]s do not take into account specific users, they are a
/// basic filter over the whole journey
pub type ServiceFilter = fn(&mut Vec<&Service>, &Journey);

/// Create a filter that removes services already visited on the journey
pub fn create_visited_filer(journey: &Journey) -> impl Fn(&&Service) -> bool {
    let visited = journey.get_visited_services();
    move |service| !visited.contains(service)
}

/// Create a filter that removes services that are down
pub fn create_down_filter() -> impl Fn(&&Service) -> bool {
    |service| service.active
}

/// Create a filter that, if there's an unmitigated CI in the journey, will remove CIs that don't
/// mitigate it
pub fn create_ci_filter(journey: &Journey) -> impl Fn(&&Service) -> bool {
    let cis = journey.get_unmitigated_cis();

    move |service| {
        // If the user has no CIs, all services are valid
        if cis.is_empty() {
            return true;
        }

        for ci in &cis {
            if service.can_mitigate_ci(ci) {
                return true;
            }
        }

        false
    }
}
