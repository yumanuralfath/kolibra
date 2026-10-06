use dioxus::prelude::*;

#[component]
pub fn auth_page() -> Element {
    rsx! {
            main {
            class: "flex-1 px-4 py-16",
                div {
                 class: "mx-auto max-w-md space-y-4",
                    h1 {
                            class: "text-center text-2xl font-bold md:text-4xl",
                            "Welcome to Kolibra"
                        }
                }
        }
    }
}
