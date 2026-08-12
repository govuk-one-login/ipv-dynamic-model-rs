use crate::service_table::cell::ServiceCell;
use data_models::prelude::{Row, Table};
use dioxus::prelude::*;

#[component]
pub fn ServiceRow(row: Row) -> Element {
    let table = use_context::<Signal<Table>>();
    let data = table()
        .get_row(row)
        .into_iter()
        .map(|service| rsx!(ServiceCell { service }))
        .collect::<Result<Vec<VNode>, _>>()?;

    rsx! {
        tr {
            th { "{row}" }
            {data.iter()}
        }
    }
}
