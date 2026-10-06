use dioxus::prelude::*;

#[component]
pub fn navbar_comp() -> Element {
    let mut theme = use_context::<Signal<bool>>();

    rsx! {
        header {
            class: "flex items-center justify-between px-4 py-4",
            h1 {
                class: "text-xl font-semibold",
                "KOLIBRA"
            }
            button {
                r#type: "button",
                class: "btn btn-ghost btn-square",
                title: if theme() { "Switch to light theme" } else { "Switch to dark theme" },
                aria_label: if theme() { "Switch to light theme" } else { "Switch to dark theme" },
                onclick: move |_| {
                    let current_theme = *theme.read();
                    *theme.write() = !current_theme;
                },
                span {
                    class: if theme() { "icon-[lucide--sun] size-5" } else { "icon-[lucide--moon] size-5" },
                    aria_hidden: "true",
                }
            }
        }
    }
}
