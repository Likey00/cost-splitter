use dioxus::prelude::*;

use crate::models::{Extra, ExtrasList};

pub fn ExtrasDisplay(cx: Scope) -> Element {
    let extras_list = use_shared_state::<ExtrasList>(cx).unwrap();

    if !extras_list.read().0.is_empty() {
        render! {
            for (idx, extra) in extras_list.read().0.iter().enumerate() {
                div {
                    class: "row",
                    SingleExtra { idx: idx, extra: extra.clone() }
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn SingleExtra(cx: Scope, idx: usize, extra: Extra) -> Element {
    let extras_list = use_shared_state::<ExtrasList>(cx).unwrap();

    render! {
        div {
            class: "input-field col s12",
            input {
                id: "extra-input-{idx}",
                class: "col s10",
                r#type: "number",
                value: "{extra.price}",
                min: "0",
                placeholder: "0",
                oninput: move |e| {
                    extras_list.write().0[*idx].price = e.value.clone();
                }
            },
            label {
                class: "active",
                r#for: "extra-input-{idx}",
                "Edit {extra.name}",
            },
            button {
                id: "extra-input-button-{idx}",
                class: "btn-floating text-center red",
                r#type: "button",
                onclick: move |_| {
                    extras_list.write().0.remove(*idx);
                },
                i {
                    class: "material-icons",
                    "remove",
                }
            }
        }
    }
}
