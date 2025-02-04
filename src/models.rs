use rocket::serde::{Deserialize, Serialize};
use rocket::form::FromForm;

/// JSON response structure for counter endpoints.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CounterResponse {
    pub name: String,
    pub count: u64,
}

/// JSON request structure for setting a counter.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CounterSetRequest {
    pub count: u64,
}

/// Query parameters for the SVG endpoint. Derives `FromForm` so Rocket can parse query parameters into this struct.
#[derive(FromForm)]
pub struct SvgOptions {
    pub label: Option<String>,
    pub color: Option<String>,
    pub style: Option<String>,
}