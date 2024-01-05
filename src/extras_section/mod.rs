use crate::extras_section::{extras_display::ExtrasDisplay, extras_form::ExtrasForm};
use dioxus::prelude::*;

mod extras_display;
mod extras_form;

pub fn ExtrasSection(cx: Scope) -> Element {
    render! {
        div {
            class: "section row card",
            div {
                class: "container",
                ExtrasDisplay {},
                ExtrasForm {},
            }
        }
    }
}
