#[macro_use]
extern crate rocket;

mod models;
mod svg_generator;

use models::{CounterResponse, CounterSetRequest, SvgOptions};
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

// Optionally load environment variables from .env.
fn init_env() {
    dotenv::dotenv().ok();
}

/// Shared counter storage.
type CounterMap = Mutex<HashMap<String, u64>>;

/// Look for the header "x-api-key" and verify that its value matches the value of the `API_KEY` environment variable.
pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(req: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let api_key = req.headers().get_one("x-api-key");
        if let Some(api_key) = api_key {
            if api_key == std::env::var("API_KEY").expect("API_KEY must be set") {
                return rocket::request::Outcome::Success(ApiKey(api_key.to_string()));
            }
        }
        rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, ()))
    }
}

/// GET endpoint to return a counter as JSON (without incrementing)
#[get("/api/counter/<name>")]
async fn get_counter_json(name: &str, counters: &State<CounterMap>) -> Json<CounterResponse> {
    let map = counters.lock().unwrap();
    let count = map.get(name).cloned().unwrap_or(0);
    Json(CounterResponse {
        name: name.to_string(),
        count,
    })
}

/// POST endpoint to increment a counter (returns the new count)
#[post("/api/counter/<name>/increment")]
async fn increment_counter_json(name: &str, counters: &State<CounterMap>) -> Json<CounterResponse> {
    let mut map = counters.lock().unwrap();
    let count = map.entry(name.to_string()).or_insert(0);
    *count += 1;
    Json(CounterResponse {
        name: name.to_string(),
        count: *count,
    })
}

/// PUT endpoint to set a counter to a given value (for administration)
/// The caller must include a valid API key in the header `x-api-key`.
#[put("/api/counter/<name>", data = "<new_value>")]
async fn set_counter_json(
    name: &str,
    new_value: Json<CounterSetRequest>,
    _api_key: ApiKey,
    counters: &State<CounterMap>,
) -> Json<CounterResponse> {
    let mut map = counters.lock().unwrap();
    map.insert(name.to_string(), new_value.count);
    Json(CounterResponse {
        name: name.to_string(),
        count: new_value.count,
    })
}

/// GET endpoint to return an SVG counter image.
/// Each time the image is requested, the counter is incremented.
/// Query parameters allow for some customization (label, color, style).
#[get("/counter/<name>/svg?<options..>")]
async fn svg_counter(
    name: &str,
    options: Option<SvgOptions>,
    counters: &State<CounterMap>,
) -> (ContentType, String) {
    // Load the base CSS from assets/style.css.
    let base_css = include_str!("../assets/style.css");
    
    // Build custom CSS if parameters are provided.
    let mut custom_css = String::new();
    if let Some(opts) = &options {
        if let Some(color) = &opts.color {
            let norm_color = if color.starts_with('#') {
                color.clone()
            } else {
                format!("#{}", color)
            };
            custom_css.push_str(&format!(":root {{ --accent-primary: {}; }}", norm_color));
        }
        if let Some(style) = &opts.style {
            custom_css.push_str(&format!(" .counter-text {{ {} }}", style));
        }
    }
    
    // Combine the base CSS with the custom CSS.
    let css = format!("{}\n{}", base_css, custom_css);
    
    let label = options
        .as_ref()
        .and_then(|opts| opts.label.clone())
        .unwrap_or_else(|| "Visits".to_string());
    
    // Increment the counter.
    let mut map = counters.lock().unwrap();
    let count = map.entry(name.to_string()).or_insert(0);
    *count += 1;
    
    // Generate the SVG
    let svg = svg_generator::generate_svg(&label, *count, &css);
    
    (ContentType::new("image", "svg+xml"), svg)
}

#[launch]
fn rocket() -> _ {
    init_env();
    rocket::build()
        .manage(Mutex::new(HashMap::<String, u64>::new()))
        .mount(
            "/",
            routes![
                svg_counter,
                get_counter_json,
                increment_counter_json,
                set_counter_json
            ],
        )
}
