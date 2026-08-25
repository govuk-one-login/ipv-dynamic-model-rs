use crate::prelude::{HasScores, Service};
use crate::user_journey::journey::Journey;

/// A [`JourneyRule`] will examine the journey a user has been on and adjust the possible
/// services accordingly. If no services are returned, it should be considered a failed
/// journey.
///
/// Note: there are probably two subtypes of rules; filters and sorts. If we could serialize this,
/// we could allow people to write and apply new rules to see what happens, and codify the policy
/// outside the compiled code.
pub type JourneyRule = for<'s> fn(&Journey, &[&'s Service]) -> Vec<&'s Service>;

/// Looks at which services a user already visited and removes them from the pool
pub const REMOVE_VISITED: JourneyRule = |journey, services| {
    let visited = journey.get_visited_services();

    services
        .iter()
        .copied()
        .filter(|s| !visited.contains(s))
        .collect()
};

/// Remove any services currently down from the list
pub const REMOVE_DOWN_SERVICES: JourneyRule =
    |_journey, services| services.iter().copied().filter(|s| s.active).collect();

/// Push any service that is degraded to the back
pub const SORT_BY_REMAINING_CAPACITY: JourneyRule = |_journey, services| {
    let mut services: Vec<_> = services.to_vec();
    services.sort_by(|left, right| {
        f64::total_cmp(&left.remaining_capacity(), &right.remaining_capacity())
    });
    services
};

/// Looks at which score is most important and largest
pub const SORT_BY_SCORE_IMPORTANCE: JourneyRule = |_journey, services| {
    let mut services: Vec<_> = services.to_vec();
    services.sort_by(|left, right| left.compare_score_types_and_scores(*right));
    services
};

/// Filters out any service not part of CI mitigation for an existing CI
pub const CI_FILTER: JourneyRule = |journey, services| {
    let cis = journey.get_unmitigated_cis();

    // If the user has no CIs, the full service list can be returned
    if cis.is_empty() {
        return services.to_vec();
    }

    // If any CIs can not be mitigated, return no services
    'ci: for ci in &cis {
        for service in services {
            if service.can_mitigate_ci(ci) {
                // The service can be mitigated we can move on
                continue 'ci;
            }
        }
        // If a ci can not be mitigated by any CI we give up
        return vec![];
    }

    // Otherwise return the services that can mitigate any of the current CIs
    services
        .iter()
        .copied()
        .filter(|service| service.can_mitigate_any_of_ci(&cis))
        .collect()
};
