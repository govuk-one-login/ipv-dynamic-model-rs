use data_models::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn ServiceCell(service: RowContent) -> Element {
    match service {
        RowContent::StartOfCri { service, rowspan } => {
            rsx! {
                td {
                    class: "{service.get_status()} {service.owner}",
                    rowspan,
                    "{service.name}"
                }
            }
        }
        RowContent::ContinuationOfCri { .. } => {
            rsx! {}
        }
        RowContent::Empty => {
            rsx! {
                td {}
            }
        }
    }
}
