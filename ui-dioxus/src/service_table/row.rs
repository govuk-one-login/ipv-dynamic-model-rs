use crate::service_table::cell::ServiceCell;
use crate::table_data::row::Row;
use crate::table_data::table::Table;
use dioxus::prelude::*;

#[component]
pub fn ServiceRow(row: Row) -> Element {
    let table = use_context::<Table>();
    let data = table
        .get_row(row)
        .into_iter()
        .map(|row_content| {
            let row_content = use_signal(|| row_content);
            rsx!(ServiceCell { row_content })
        })
        .collect::<Result<Vec<VNode>, _>>()?;

    rsx! {
        tr {
            th { "{row}" }
            {data.iter()}
        }
    }
}
