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
