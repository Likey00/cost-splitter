use crate::models::{ItemList, NameList};
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

pub fn NameForm(cx: Scope) -> Element {
    let curr_name = use_state(cx, || "".to_owned());
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let item_list = use_shared_state::<ItemList>(cx).unwrap();

    let add_name = || {
        let n = curr_name.get().clone();
        if n.is_empty() {
            return;
        }
        if !name_list.read().0.contains(&n) {
            name_list.write().0.push(n);
        }

        for item in item_list.write().0.iter_mut() {
            item.people.push(false);
        }

        curr_name.set("".to_owned());
    };

    render! {
        div {
            class: "input-field col s12",
            input {
                id: "name-input",
                class: "col s10",
                r#type: "text",
                placeholder: "Add People",
                oninput: move |e| {
                    curr_name.set(e.value.clone());
                },
                value: "{curr_name.get()}",
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        add_name();
                    }
                }
            },
            button {
                id: "name-button",
                class: "btn-floating text-center green",
                r#type: "button",
                onclick: move |_| add_name(),
                i {
                    class: "material-icons",
                    "add"
                }
            },
        }
    }
}
