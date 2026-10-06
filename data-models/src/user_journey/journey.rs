use crate::identity::Proofing;
use crate::prelude::*;
use crate::user_journey::ci::Ci;
use crate::user_journey::journey_step::JourneyStep;
use crate::user_journey::rule::service_filter::{
    create_ci_filter, create_down_filter, create_visited_filer,
};
use crate::user_journey::rule::service_weights::{
    ServiceWeight, create_weigh_by_remaining_capacity,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum JourneyOutcome {
    #[default]
    Ongoing,
    Completed(KnownIdentityProfile),
    Failed,
}

impl JourneyOutcome {
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        match self {
            Self::Ongoing => false,
            Self::Completed(_) | Self::Failed => true,
        }
    }
}

/// A `Journey` represents a single path that a user is taking through the system
#[derive(Debug, Clone, PartialEq)]
pub struct Journey {
    users: Users,
    steps: Vec<JourneyStep>,
    goal: Proofing,
    outcome: JourneyOutcome,
}

impl Journey {
    #[must_use]
    pub fn new(users: Users, goal: Proofing) -> Self {
        Self {
            users,
            steps: Vec::with_capacity(20), // Cheating a bit but this number should be more than the most steps they can take
            goal,
            outcome: JourneyOutcome::default(),
        }
    }

    #[must_use]
    pub const fn get_users(&self) -> &Users {
        &self.users
    }

    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.outcome.is_complete()
    }

    pub fn possible_next_steps(&self) {
        todo!()
    }

    #[must_use]
    pub fn get_visited_services(&self) -> Vec<&Service> {
        self.steps.iter().map(JourneyStep::get_service).collect()
    }

    #[must_use]
    pub fn get_unmitigated_cis(&self) -> Vec<&Ci> {
        self.users.get_unmitigated_cis()
    }

    /// Consumes the journey to find the next steps
    ///
    /// This will always result in multiple branches as some users may be sent to different
    /// services, and some users may fail at the service they were sent to.
    ///
    /// If the journey has a [`CompletedStatus`], calling this method return a new Vec with the same
    /// journey inside it.
    #[must_use]
    pub fn step(self, services: &[&Service]) -> Vec<Self> {
        if self.is_complete() {
            return vec![self];
        }

        let mut possible_services_weighted: Vec<_> = services
            .iter()
            .copied()
            // First remove unusable services
            .filter(create_down_filter())
            .filter(create_visited_filer(&self))
            .filter(create_ci_filter(&self))
            // Next we'll swap to the weighted type for further processing
            .map(ServiceWeight::from)
            .collect();

        // If the journey was incomplete and there is nowhere to send people, then they have failed
        if possible_services_weighted.is_empty() {
            return vec![Self {
                outcome: JourneyOutcome::Failed,
                ..self
            }];
        }

        // Apply some weightings
        create_weigh_by_remaining_capacity(1.0)(&mut possible_services_weighted);

        // Finally sort by the weight
        possible_services_weighted
            .sort_by(|left, right| f64::total_cmp(&left.get_weight(), &right.get_weight()));

        // Finally we now need to work out who of our users can go to which service. Every user must
        // be accounted for.
        let mut users = self.get_users().clone();
        let branches = Vec::new();
        let steps = self.steps;

        // ToDo: First pass look for all users who can go to unburdened servers.

        // for service_weight in possible_services_weighted {
        //     let service = service_weight.get_service();
        //
        //     match service.user_requirement {
        //         None => {
        //             // No requirement, but each service will have a
        //             branches.push(JourneyStep::Success(service));
        //             branches.push(Self {
        //                 users,
        //                 steps,
        //                 goal: Proofing::P1,
        //                 outcome: Default::default(),
        //             })
        //         }
        //         Some(requirement) => {}
        //     }
        // }

        branches
    }
}
