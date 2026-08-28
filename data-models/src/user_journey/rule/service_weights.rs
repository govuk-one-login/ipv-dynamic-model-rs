use crate::prelude::Service;

#[derive(Copy, Clone)]
pub struct ServiceWeight<'s> {
    service: &'s Service,
    weight: f64,
}

impl<'s> From<&'s Service> for ServiceWeight<'s> {
    fn from(service: &'s Service) -> Self {
        ServiceWeight {
            service,
            weight: 1.0,
        }
    }
}

/// Push any service that is degraded to the back
#[allow(
    clippy::cast_precision_loss,
    reason = "We're unlikely to have 2^52 services"
)]
pub fn create_sort_by_remaining_capacity(scale: f64) -> impl Fn(&mut Vec<ServiceWeight>) {
    move |services: &mut Vec<ServiceWeight>| {
        // Sort from decreasing capacity to most capacity
        services.sort_by(|left, right| {
            f64::total_cmp(
                &right.service.remaining_capacity(),
                &left.service.remaining_capacity(),
            )
        });
        for (pos, service_weight) in services.iter_mut().enumerate() {
            service_weight.weight *= scale * pos as f64;
        }
    }
}
