use crate::{models::NameList, summary_section::one_person::OnePerson};
use dioxus::prelude::*;

mod one_person;

pub fn SummarySection(cx: Scope) -> Element {
    let name_list = use_shared_state::<NameList>(cx).unwrap();
    let num_names = name_list.read().0.len();

    render! {
        if num_names != 0 {
            rsx! {
                div {
                    class: "section row card",
                    div {
                        class: "container",
                        for idx in 0..num_names {
                            OnePerson { idx: idx }
                        }
                    }
                }
            }
        }
    }
}
