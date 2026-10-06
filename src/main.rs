use dioxus::{logger::tracing::Level, prelude::*};
use ui::Home;

mod core;
mod server;
mod ui;

const FAVICON: Asset = asset!("../assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MAIN_CSS: Asset = asset!("../assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home{},
}

fn main() {
    #[cfg(feature = "server")]
    core::load_env();

    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| false);
    use_context_provider(|| theme);

    rsx! {
        document::Link {rel: "icon", href: FAVICON}
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {rel: "stylesheet", href: MAIN_CSS}
        div {
            "data-theme": if theme() { "dark" } else { "light" },
            Router<Route> {}
        }
    }
}
