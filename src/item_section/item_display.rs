use dioxus::prelude::*;

use crate::{
    item_section::single_item_form::SingleItemForm,
    models::{Item, ItemList},
};

pub fn ItemDisplay(cx: Scope) -> Element {
    let item_list = use_shared_state::<ItemList>(cx).unwrap();

    if !item_list.read().0.is_empty() {
        render! {
            ul {
                class: "collection",
                for (idx, item) in item_list.read().0.iter().enumerate() {
                    SingleItem { idx: idx, item: item.clone() }
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn SingleItem(cx: Scope, idx: usize, item: Item) -> Element {
    let item_list = use_shared_state::<ItemList>(cx).unwrap();
    let class = format!(
        "collection-item black-text {}",
        if item.active { "active" } else { "" }
    );
    let activate_icon = if item.active {
        "expand_less"
    } else {
        "expand_more"
    };

    let price_value = if item.price == "" {
        0.
    } else {
        item.price.parse::<f64>().unwrap()
    };

    render! {
        li {
            class: "{class}",
            button {
                id: "item-delete-btn-{idx}",
                class: "secondary-content btn-small btn-floating red white-text",
                onclick: move |_| {
                    item_list.write().0.remove(*idx);
                },
                i {
                    class: "material-icons",
                    "remove"
                }
            },
            button {
                id: "item-copy-btn-{idx}",
                class: "secondary-content btn-small btn-floating orange white-text",
                onclick: move |_| {
                    item_list.write().0.insert(*idx+1, item.clone());
                    item_list.write().0[*idx].active = false;
                },
                i {
                    class: "material-icons",
                    "content_copy",
                }
            },
            button {
                id: "item-activate-btn-{idx}",
                class: "secondary-content btn-small btn-floating blue white-text",
                onclick: move |_| {
                    if item_list.read().0.len() <= *idx {
                        return;
                    }
                    let curr_active = item_list.read().0[*idx].active;
                    item_list.write().0[*idx].active = !curr_active;
                },
                i {
                    class: "material-icons",
                    activate_icon,
                }
            },
            b { "{item.name}" },
            br {},
            "${price_value:.2} split between {item.count}"
        },
        if item.active {
            rsx! { SingleItemForm { idx: *idx, item: item.clone() } }
        }
    }
}
