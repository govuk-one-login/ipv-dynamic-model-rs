#![allow(clippy::volatile_composites)]

use data_models::prelude::*;
use dioxus::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::rc::Rc;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn EnvironmentControls() -> Element {
    rsx! {
        form {
            label {
                "Users per Second"
                input { size: "3", "10" }
            }
        }
    }
}

#[component]
fn UserControls() -> Element {
    rsx! {
        form {
            label {
                "Passport"
                input { r#type: "checkbox", checked: "checked" }
            }
            label {
                "Driving License"
                input { r#type: "checkbox", checked: "checked" }
            }
        }
    }
}

#[component]
fn SuccessRate(success_rate: f64) -> Element {
    let percent = success_rate * 100.0;
    rsx! {
        h2 { "Success rate: {percent:.1}%" }
    }
}

#[component]
fn DynamicJourneyTable() -> Element {
    let mut rows = Vec::new();

    for row in Column::row_order() {
        rows.push(rsx!(DynamicJourneyRow { row })?);
    }

    rsx! {
        table { {rows.iter()} }
    }
}

#[component]
fn DynamicJourneyRow(row: Row) -> Element {
    let table = use_context::<Table>();
    let mut row_data = Vec::new();
    for cri in table.get_row(row) {
        let data = match cri {
            None => rsx!(td {})?,
            Some(cri) => rsx!(
                td { "{cri.name}" }
            )?,
        };
        row_data.push(data);
    }

    rsx! {
        tr {
            th { "{row}" }
            {row_data.iter()}
        }
    }
}

#[component]
fn App() -> Element {
    let mut cris = Vec::new();
    for document in yaml_serde::Deserializer::from_reader(File::open("../test-data/test.yaml")?) {
        cris.push(Rc::new(Cri::deserialize(document)?));
    }
    use_context_provider(|| Table::new(cris));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
        EnvironmentControls {}
        UserControls {}
        DynamicJourneyTable {}
        SuccessRate { success_rate: 0.4 }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            h1 { "Dynamic Journeys Playground" }
        }
    }
}
