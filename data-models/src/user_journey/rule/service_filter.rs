use crate::prelude::Service;
use crate::user_journey::journey::Journey;

/// As we step through a user [`Journey`] and visit different services, not all services will be
/// appropriate to visit. [`ServiceFilter`]s do not take into account specific users, they are a
/// basic filter over the whole journey
pub type ServiceFilter = fn(&mut Vec<&Service>, &Journey);

/// Looks at which services a user already visited and removes them from the pool
pub const FILTER_VISITED: ServiceFilter = |services, journey| {
    let visited = journey.get_visited_services();
    services.retain(|s| !visited.contains(s));
};

/// Remove any services currently down from the list
pub const FILTER_DOWN_SERVICES: ServiceFilter = |services, _journey| services.retain(|s| s.active);

/// Filters out any service not part of CI mitigation for an existing CI. If there are no CIs all
/// service will remain
pub const FILTER_CI: ServiceFilter = |services, journey| {
    let cis = journey.get_unmitigated_cis();

    // If the user has no CIs, we won't touch the existing list
    if cis.is_empty() {
        return;
    }

    // If any CIs can not be mitigated, clear all services, there is nowhere to go
    'ci: for ci in &cis {
        for service in services.iter() {
            if service.can_mitigate_ci(ci) {
                // The service can be mitigated we can move on
                continue 'ci;
            }
        }
        // If a ci can not be mitigated by any CI we give up
        services.clear();
        return;
    }

    // Otherwise return the services that can mitigate any of the current CIs
    services.retain(|service| service.can_mitigate_any_of_ci(&cis));
};
