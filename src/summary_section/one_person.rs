use dioxus::prelude::*;

use crate::models::{Extra, ExtrasList, Item, ItemList, NameList};

#[component]
pub fn OnePerson(cx: Scope, idx: usize) -> Element {
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let item_list = use_shared_state::<ItemList>(cx).unwrap();
    let extras_list = use_shared_state::<ExtrasList>(cx).unwrap();

    let name = name_list.read().0[*idx].clone();
    let relevant_items: Vec<Item> = item_list
        .read()
        .0
        .clone()
        .into_iter()
        .filter(|item| item.people[*idx] && item.price != "0" && item.price != "")
        .collect();
    let mut relevant_extras: Vec<Extra> = extras_list
        .read()
        .0
        .clone()
        .into_iter()
        .filter(|extra| extra.price != "0" && extra.price != "")
        .collect();
    if relevant_items.len() == 0 {
        relevant_extras = Vec::new();
    }

    let total_items_price: f64 = item_list
        .read()
        .0
        .iter()
        .map(|item| {
            if item.price != "" {
                item.price.parse::<f64>().unwrap()
            } else {
                0.
            }
        })
        .sum();
    let total_extras_price: f64 = relevant_extras
        .iter()
        .map(|extra| {
            if extra.price != "" {
                extra.price.parse::<f64>().unwrap()
            } else {
                0.
            }
        })
        .sum();

    let items_price: f64 = relevant_items
        .iter()
        .map(|item| item.price.parse::<f64>().unwrap() / item.count as f64)
        .sum();
    let fraction = if total_items_price == 0. {
        0.
    } else {
        items_price / total_items_price
    };
    let price_after_extras = items_price + total_extras_price * fraction;

    render! {
        ul {
            class: "collection with-header",
            li {
                class: "collection-header",
                b {
                    name.clone(),
                    span {
                        class: "right",
                        "${price_after_extras:.2}",
                    },
                },
            },
            for item in relevant_items {
                li {
                    class: "collection-item",
                    "1/{item.count} {item.name}",
                    span {
                        class: "right",
                        "${item.price.parse::<f64>().unwrap() / item.count as f64 :.2}"
                    }
                }
            },
            for extra in relevant_extras {
                li {
                    class: "collection-item",
                    "{extra.name}",
                    span {
                        class: "right",
                        "${extra.price.parse::<f64>().unwrap() * fraction :.2}"
                    }
                }
            }
        }
    }
}
