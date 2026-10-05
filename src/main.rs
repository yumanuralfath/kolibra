use dioxus::prelude::*;

mod components;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        main {
            h1 { "Login" }
            form {
                onsubmit: move |event| event.prevent_default(),
                label { r#for: "email", "Email" }
                input {
                    id: "email",
                    name: "email",
                    r#type: "email",
                    autocomplete: "email",
                    required: true,
                }
                label { r#for: "password", "Password" }
                input {
                    id: "password",
                    name: "password",
                    r#type: "password",
                    autocomplete: "current-password",
                    required: true,
                }
                button { r#type: "submit", "Login" }
            }
        }
    }
}
