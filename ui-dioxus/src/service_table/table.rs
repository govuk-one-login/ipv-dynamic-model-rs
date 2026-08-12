use dioxus::prelude::*;
use data_models::prelude::*;
use crate::service_table::row::ServiceRow;

#[component]
pub fn ServiceTable() -> Element {
    let mut rows = Vec::new();

    for row in Column::row_order() {
        rows.push(rsx!(ServiceRow { row })?);
    }

    rsx! {
        table {
            class: "service-table",
            {rows.iter()}
        }
    }}
