use dioxus::prelude::*;

use crate::models::{Item, ItemList, NameList};

#[component]
pub fn SingleItemForm(cx: Scope, idx: usize, item: Item) -> Element {
    let item_list = use_shared_state::<ItemList>(cx).unwrap();

    render! {
        div {
            class: "collection-item",
            div {
                class: "input-field col s12",
                input {
                    id: "item-name-{idx}",
                    r#type: "text",
                    value: "{item.name}",
                    oninput: move |e| {
                        item_list.write().0[*idx].name = e.value.clone();
                    }
                },
                label {
                    class: "active",
                    r#for: "item-name-{idx}",
                    "Edit Item Name",
                },
            },
            div {
                class: "input-field col s12",
                input {
                    id: "item-price-{idx}",
                    r#type: "number",
                    min: "0",
                    placeholder: "0",
                    value: "{item.price}",
                    oninput: move |e| {
                        item_list.write().0[*idx].price = e.value.clone();
                    },
                },
                label {
                    class: "active",
                    r#for: "item-price-{idx}",
                    "Edit Item Price",
                },
            },
            NameCheckboxes { idx: *idx, item: item.clone() }
        }
    }
}

#[component]
pub fn NameCheckboxes(cx: Scope, idx: usize, item: Item) -> Element {
    let item_list = use_shared_state::<ItemList>(cx).unwrap();
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let check_all = item_list.read().0[*idx].count == name_list.read().0.len();

    render! {
        div {
            class: "container",
            label {
                onchange: move |_| {
                    for nidx in 0..name_list.read().0.len() {
                        if !item.people[nidx] {
                            item_list.write().0[*idx].count += 1;
                        }
                        item_list.write().0[*idx].people[nidx] = true;
                    }
                },
                input {
                    r#type: "checkbox",
                    checked: check_all,
                    disabled: check_all,
                },
                span { "Select All" },
            },
            br {},
            for (nidx, name) in name_list.read().0.iter().enumerate() {
                rsx! {
                    label {
                        onchange: move |_| {
                            let checked = item.people[nidx];
                            item_list.write().0[*idx].people[nidx] = !checked;
                            if checked {
                                item_list.write().0[*idx].count -= 1;
                            } else {
                                item_list.write().0[*idx].count += 1;
                            }
                        },
                        input {
                            r#type: "checkbox",
                            checked: item_list.read().0[*idx].people[nidx],
                        },
                        span { "{name}" },
                    },
                    br {},
                }
            }
        }
    }
}
