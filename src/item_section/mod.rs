use crate::item_section::{item_display::ItemDisplay, item_form::ItemForm};
use dioxus::prelude::*;

mod item_display;
mod item_form;
mod single_item_form;

pub fn ItemSection(cx: Scope) -> Element {
    render! {
        div {
            class: "section row card",
            div {
                class: "container",
                ItemDisplay {},
                ItemForm {},
            }
        }
    }
}
