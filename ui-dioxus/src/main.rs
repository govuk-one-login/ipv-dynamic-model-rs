#![allow(
    clippy::volatile_composites,
    reason = "Required by Dioxus `asset!` macro"
)]

mod service_table;
mod table_data;
mod users_panel;

use crate::service_table::table::ServiceTable;
use crate::table_data::table::Table;
use crate::users_panel::UsersPanel;
use data_models::prelude::*;
use dioxus::prelude::*;
use serde::Deserialize;
use std::fs::File;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");

fn main() {
    launch(App);
}

#[component]
fn SuccessRate(success_rate: f64) -> Element {
    let percent = success_rate * 100.0;
    rsx! {
        h2 { "Success rate: {percent:.1}%" }
    }
}

#[component]
fn MainBody() -> Element {
    rsx! {
        div {
            class: "main",
            UsersPanel {}
            ServiceTable {}
        }
    }
}

#[component]
fn App() -> Element {
    let mut cris = Vec::new();
    for document in yaml_serde::Deserializer::from_reader(File::open("../test-data/test.yaml")?) {
        cris.push(use_signal(|| {
            Service::new(Cri::deserialize(document).expect("Error parsing data"))
        }));
    }
    // Note: Using a signal here adds some overhead, both cognitively (using signals adds extra
    // steps), and in performance (it'll redraw all components that consume it on change), but I'm
    // not sure what the best way to mutate the table is
    use_context_provider(|| Table::new(cris));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
        MainBody {}
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
