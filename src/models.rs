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
#[derive(FromForm, Clone, Default)]
pub struct SvgOptions {
    pub label: Option<String>,
    pub style: Option<String>,
    // SVG Dimensions
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub label_width: Option<u32>,
    pub counter_width: Option<u32>,
    pub radius: Option<u32>,
    // Gradient Settings
    pub grad_stop1_color: Option<String>,
    pub grad_stop1_opacity: Option<f32>,
    pub grad_stop2_opacity: Option<f32>,
    // Text Settings
    pub font_family: Option<String>,
    pub font_size: Option<u32>,
    pub label_offset_x: Option<u32>,
    pub label_offset_y: Option<u32>,
    pub counter_offset_x: Option<u32>,
    pub counter_offset_y: Option<u32>,
    pub shadow_fill: Option<String>,
    pub shadow_opacity: Option<f32>,
    // Color Settings
    pub background_label: Option<String>,
    pub background_counter: Option<String>,
    pub label_color: Option<String>,
    pub counter_color: Option<String>,
}
