use crate::core::Route;
use crate::server::auth::{check_session, login_password};
use dioxus::prelude::*;

#[component]
pub fn AuthPage() -> Element {
    let nav = use_navigator();
    let mut password = use_signal(String::new);
    let mut authenticated = use_signal(|| None::<bool>);
    let mut submitting = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let session = use_resource(|| async { check_session().await });
    use_effect(move || {
        let server_ok = session
            .read()
            .as_ref()
            .is_some_and(|r| matches!(r, Ok(true)));
        if authenticated().unwrap_or(server_ok) {
            nav.replace(Route::Dashboard {});
        }
    });
    rsx! {
        main { class: "flex-1 px-4 py-16",
            div { class: "mx-auto max-w-md space-y-6",
                h1 { class: "text-center text-2xl font-bold md:text-4xl", "Welcome Reader" }
                p { class: "text-center text-base leading-relaxed",
                    "Wanna see your read stat with visualize"
                }
                form {
                    class: "space-y-4",
                    onsubmit: move |event| {
                        event.prevent_default();
                        submitting.set(true);
                        let entered_password = password();
                        spawn(async move {
                            match login_password(entered_password).await {
                                Ok((_, result)) if result.0 => {
                                    authenticated.set(Some(true));
                                    password.set(String::new());
                                    message.set(None);
                                    nav.replace(Route::Dashboard {});
                                }
                                Ok((_, _)) => {
                                    authenticated.set(Some(false));
                                    message
                                        .set(Some("Password not valid or auth dont config".to_string()));
                                }
                                Err(_) => message.set(Some("Login failed please try again".to_string())),
                            }
                            submitting.set(false);
                        });
                    },
                    label { class: "block space-y-2",
                        span { class: "text-sm font-medium", "Password" }
                        input {
                            class: "input input-bordered w-full",
                            r#type: "password",
                            name: "password",
                            autocomplete: "current-password",
                            required: true,
                            value: "{password}",
                            oninput: move |event| password.set(event.value()),
                        }
                    }
                    if let Some(error) = message() {
                        p { class: "text-sm text-error", role: "alert", "{error}" }
                    }
                    button {
                        class: "btn btn-primary w-full",
                        r#type: "submit",
                        disabled: submitting(),
                        if submitting() {
                            "Checking..."
                        } else {
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
