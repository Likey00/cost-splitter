use dioxus::prelude::*;

use crate::name_section::{name_form::NameForm, name_tags::NameTags};

mod name_form;
mod name_tags;

pub fn NameSection(cx: Scope) -> Element {
    render! {
        div {
            class: "section row card",
            div {
                class: "container",
                NameTags {},
                NameForm {},
            }
        }
    }
}
