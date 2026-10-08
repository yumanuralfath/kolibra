use dioxus::prelude::*;

#[component]
pub fn footer_comp(year: u16, name: String) -> Element {
    rsx! {
        footer { class: "px-4 py-4 text-center text-sm",
            span {
                "(c) {year} {name}. Build with "

                span { class: "text-rotate",
                    span {
                        span { class: "bg-teal-400 text-teal-800 px-2", "Dioxus" }
                        span { class: "bg-red-400 text-red-800 px-2", "Heart" }
                    }
                }
            }
        }
    }
}
