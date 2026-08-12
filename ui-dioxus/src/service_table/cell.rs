use data_models::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn ServiceCell(service: Option<Service>) -> Element {
    match service {
        None => rsx!(td {}),
        Some(service) => {
            rsx!(
                td {
                    class: "{service.get_status()} {service.owner}",
                    "{service.name}"
                }
            )
        }
    }
}
