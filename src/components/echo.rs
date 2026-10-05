use chrono::{DateTime, FixedOffset, Utc};
use dioxus::{fullstack::Json, prelude::*};
use serde::{Deserialize, Serialize};

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
