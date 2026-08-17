use crate::table_data::column::RowContent;
use dioxus::prelude::*;

#[component]
pub fn ServiceCell(row_content: Signal<RowContent>) -> Element {
    match row_content() {
        RowContent::StartOfCri {
            mut service,
            rowspan,
        } => {
            let s = service();
            rsx! {
                td {
                    class: "{s.get_status()} {s.owner}",
                    rowspan,
                    label {
                        class: "system",
                        "{s.name}"

                        input {
                            r#type: "checkbox",
                            checked: "{s.get_active()}",
                            onchange: move |event| {
                                match event.value().as_str() {
                                    "true" => service.write().turn_on(),
                                    "false" => service.write().turn_off(),
                                    _ => {},
                                }
                            }
                        }
                    }
                }
            }
        }
        // Continuation doesn't include anything as the row above it will have been expanded
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
