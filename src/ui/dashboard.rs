use crate::core::Route;
use crate::server::auth::{check_session, logout};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let nav = use_navigator();
    let session = use_resource(|| async { check_session().await });
    use_effect(move || {
        if let Some(Ok(false)) | Some(Err(_)) = session.read().as_ref() {
            nav.replace(Route::Home {});
        }
    });
    rsx! {
        button {
            class: "btn btn-outline",
            onclick: move |_| {
                spawn(async move {
                    if logout().await.is_ok() {
                        nav.replace(Route::Home {});
                    }
                });
            },
            "Logout"
        }
    }
}
