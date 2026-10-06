use dioxus::prelude::*;

#[component]
pub fn footer_comp(year: u16, name: String) -> Element {
    rsx! {
        footer {
            class: "px-4 py-4",
            h3 {
                class:"text-center",
                "Copyright (c) {year} {name}. All Rights Reserved."
            }
        }
    }
}
