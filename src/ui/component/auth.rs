use crate::server::auth::{check_session, login_password, logout};
use dioxus::prelude::*;

#[component]
pub fn auth_page() -> Element {
    let mut password = use_signal(String::new);
    let mut authenticated = use_signal(|| None::<bool>);
    let mut submitting = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let session = use_resource(|| async { check_session().await });
    let server_authenticated = session
        .read()
        .as_ref()
        .is_some_and(|result| matches!(result, Ok(true)));
    let signed_in = authenticated().unwrap_or(server_authenticated);

    rsx! {
        main {
            class: "flex-1 px-4 py-16",
            div {
                class: "mx-auto max-w-md space-y-6",
                h1 {
                    class: "text-center text-2xl font-bold md:text-4xl",
                    "Welcome Reader"
                }
                p {
                    class: "text-center text-base leading-relaxed",
                    "Wanna see your read stat with visualize"
                }
                if signed_in {
                    div {
                        class: "space-y-4 text-center",
                        p { "Login berhasil. Selamat membaca." }
                        if let Some(error) = message() {
                            p {
                                class: "text-sm text-error",
                                role: "alert",
                                "{error}"
                            }
                        }
                        button {
                            class: "btn btn-outline",
                            disabled: submitting(),
                            onclick: move |_| {
                                submitting.set(true);
                                spawn(async move {
                                    match logout().await {
                                        Ok((_, _)) => {
                                            authenticated.set(Some(false));
                                            message.set(None);
                                        }
                                        Err(_) => message.set(Some("Logout gagal. Coba lagi.".to_string())),
                                    }
                                    submitting.set(false);
                                });
                            },
                            "Logout"
                        }
                    }
                } else {
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
                                    }
                                    Ok((_, _)) => {
                                        authenticated.set(Some(false));
                                        message.set(Some("Password tidak valid atau konfigurasi login server belum tersedia.".to_string()));
                                    }
                                    Err(_) => message.set(Some("Login gagal. Coba lagi.".to_string())),
                                }
                                submitting.set(false);
                            });
                        },
                        label {
                            class: "block space-y-2",
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
                            p {
                                class: "text-sm text-error",
                                role: "alert",
                                "{error}"
                            }
                        }
                        button {
                            class: "btn btn-primary w-full",
                            r#type: "submit",
                            disabled: submitting(),
                            if submitting() { "Memeriksa..." } else { "Login" }
                        }
                    }
                }
            }
        }
    }
}
