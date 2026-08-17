use crate::service_table::row::ServiceRow;
use crate::table_data::column::Column;
use dioxus::prelude::*;

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
    }
}
