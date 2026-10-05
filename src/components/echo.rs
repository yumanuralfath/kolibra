use chrono::{DateTime, FixedOffset, Utc};
use dioxus::{fullstack::Json, prelude::*};
use serde::{Deserialize, Serialize};

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[post]`/`#[get]` attributes, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    Ok(input)
}

#[derive(Debug, Deserialize, Serialize)]
struct CheckHelth {
    success: bool,
    date: DateTime<FixedOffset>,
}

#[get("/api/health")]
async fn health_check() -> Result<Json<CheckHelth>> {
    let utc_now = Utc::now();
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_now = utc_now.with_timezone(&wib_offset);

    Ok(Json(CheckHelth {
        success: true,
        date: wib_now,
    }))
}
