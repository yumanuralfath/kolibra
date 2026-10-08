mod init;

use crate::ui::{Dashboard, Home};
use dioxus::prelude::*;
pub use init::init;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

#[component]
pub fn App() -> Element {
    let theme = use_signal(|| false);
    use_context_provider(|| theme);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { "data-theme": if theme() { "dark" } else { "light" }, Router::<Route> {} }
    }
}
