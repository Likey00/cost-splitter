use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::models::{Item, ItemList, NameList};

pub fn ItemForm(cx: Scope) -> Element {
    let item_name = use_state(cx, || "".to_owned());
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let item_list = use_shared_state::<ItemList>(cx).unwrap();

    let add_item = || {
        let name = item_name.get().clone();
        if name.is_empty() {
            return;
        }

        let new_item = Item {
            name,
            price: "".to_owned(),
            people: vec![false; name_list.read().0.len()],
            count: 0,
            active: false,
        };

        item_list.write().0.push(new_item);
        item_name.set("".to_owned());
    };

    render! {
        div {
            class: "input-field col s12",
            input {
                id: "item-input",
                class: "col s10",
                r#type: "text",
                placeholder: "Add Items",
                oninput: move |e| {
                    item_name.set(e.value.clone());
                },
                value: "{item_name.get()}",
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        add_item();
                    }
                }
            },
            button {
                id: "item-button",
                class: "btn-floating text-center green",
                r#type: "button",
                onclick: move |_| add_item(),
                i {
                    class: "material-icons",
                    "add"
                }
            },
        }
    }
}
