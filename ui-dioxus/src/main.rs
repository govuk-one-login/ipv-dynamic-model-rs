#![allow(clippy::volatile_composites)]

mod service_table;

use crate::service_table::table::ServiceTable;
use data_models::prelude::*;
use dioxus::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::rc::Rc;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const SERVICE_TABLE_CSS: Asset = asset!("/assets/service_table.scss");

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
fn App() -> Element {
    let mut cris = Vec::new();
    for document in yaml_serde::Deserializer::from_reader(File::open("../test-data/test.yaml")?) {
        cris.push(Rc::new(Cri::deserialize(document)?));
    }
    // Note: Using a signal here adds some overhead, both cognitively (using signals adds extra
    // steps), and in performance (it'll redraw all components that consume it on change), but I'm
    // not sure what the best way to mutate the table is
    use_context_provider(|| Signal::new(Table::new(cris)));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: SERVICE_TABLE_CSS }
        Hero {}
        EnvironmentControls {}
        UserControls {}
        ServiceTable {}
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
