#![allow(non_snake_case)]
use crate::extras_section::ExtrasSection;
use crate::item_section::ItemSection;
use crate::models::{Extra, ExtrasList, ItemList, NameList};
use crate::name_section::NameSection;
use crate::summary_section::SummarySection;
use dioxus::prelude::*;
use log::LevelFilter;

mod extras_section;
mod item_section;
mod models;
mod name_section;
mod summary_section;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || NameList(Vec::new()));
    use_shared_state_provider(cx, || ItemList(Vec::new()));
    use_shared_state_provider(cx, || {
        ExtrasList(vec![
            Extra {
                name: "Tax".to_owned(),
                price: "".to_owned(),
            },
            Extra {
                name: "Tip".to_owned(),
                price: "".to_owned(),
            },
        ])
    });

    render! {
        div {
            class: "container",
            h3 { "Cost Splitter" },
            div { class: "divider" },
            div {
                class: "section row",
                div {
                    class: "col s12 m6",
                    NameSection {},
                    ItemSection {},
                    ExtrasSection {},
                },
                div {
                    class: "col s12 m5 right",
                    SummarySection {},
                }
            },
        }
    }
}
