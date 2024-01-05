use crate::models::{ItemList, NameList};
use dioxus::prelude::*;
use log::info;

pub fn NameTags(cx: Scope) -> Element {
    let name_list = use_shared_state::<NameList>(cx).unwrap();

    if !name_list.read().0.is_empty() {
        render! {
            for (idx, name) in name_list.read().0.iter().enumerate() {
                NameTag { idx: idx, name: name.clone() }
            }
            br {}
        }
    } else {
        None
    }
}

#[component]
fn NameTag(cx: Scope, idx: usize, name: String) -> Element {
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let item_list = use_shared_state::<ItemList>(cx).unwrap();

    render! {
        div {
            class: "chip",
            name.clone(),
            i {
                class: "close material-icons",
                onclick: move |_| {
                    name_list.write().0.remove(*idx);
                    for item in item_list.write().0.iter_mut() {
                        item.count = item.count - item.people[*idx] as usize;
                        item.people.remove(*idx);
                    }
                    info!("{:?}", item_list.read().0);
                },
                "close"
            }
        }
    }
}
