use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::models::{Extra, ExtrasList};

pub fn ExtrasForm(cx: Scope) -> Element {
    let extra_name = use_state(cx, || "".to_owned());
    let extras_list = use_shared_state::<ExtrasList>(cx).unwrap();

    let add_extra = || {
        let name = extra_name.get().clone();
        if name.is_empty() {
            return;
        }

        let new_item = Extra {
            name,
            price: "".to_owned(),
        };

        extras_list.write().0.push(new_item);
        extra_name.set("".to_owned());
    };

    render! {
        div {
            class: "input-field col s12",
            input {
                id: "extras-input",
                class: "col s10",
                r#type: "text",
                placeholder: "Add Extra Charges",
                oninput: move |e| {
                    extra_name.set(e.value.clone());
                },
                value: "{extra_name.get()}",
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        add_extra();
                    }
                }
            },
            button {
                id: "item-button",
                class: "btn-floating text-center green",
                r#type: "button",
                onclick: move |_| add_extra(),
                i {
                    class: "material-icons",
                    "add"
                }
            },
        }
    }
}
