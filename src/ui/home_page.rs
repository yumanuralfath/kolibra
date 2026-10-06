use dioxus::prelude::*;

use crate::ui::component::{auth_page, footer_comp, navbar_comp};

const APP_NAME: &str = "Yumana";
const APP_YEAR: u16 = 2026;

#[component]
pub fn Home() -> Element {
    rsx! {
    div {
        class: "flex min-h-screen flex-col",

            navbar_comp {  }

            auth_page { }

            footer_comp {
                name: APP_NAME.to_string(),
                year: APP_YEAR
                }
        }
    }
}
