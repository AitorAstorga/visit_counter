#[macro_use]
extern crate rocket;

mod models;
mod persistent_counter;
mod svg_generator;

use std::io::Cursor;

use models::{ApiKey, CounterResponse, CounterSetRequest, SvgOptions, SvgResponse};
use persistent_counter::PersistentCounterMap;

use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{Response, State};

use svg_generator::build_custom_css;

// Optionally load environment variables from .env.
fn init_env() {
    dotenv::dotenv().ok();
}

/// GET endpoint to return a counter as JSON (without incrementing)
#[get("/api/counter/<name>")]
async fn get_counter_json(name: &str, counters: &State<PersistentCounterMap>) -> Json<CounterResponse> {
    let count = counters.get(name);
    Json(CounterResponse {
        name: name.to_string(),
        count,
    })
}

/// POST endpoint to increment a counter (returns the new count)
#[post("/api/counter/<name>/increment")]
async fn increment_counter_json(name: &str, counters: &State<PersistentCounterMap>) -> Json<CounterResponse> {
    let count = counters.increment(name);
    Json(CounterResponse {
        name: name.to_string(),
        count,
    })
}

/// PUT endpoint to set a counter to a given value (for administration)
/// The caller must include a valid API key in the "x-api-key" header.
#[put("/api/counter/<name>", data = "<new_value>")]
async fn set_counter_json(
    name: &str,
    new_value: Json<CounterSetRequest>,
    _api_key: ApiKey,
    counters: &State<PersistentCounterMap>,
) -> Json<CounterResponse> {
    counters.set(name, new_value.count);
    Json(CounterResponse {
        name: name.to_string(),
        count: new_value.count,
    })
}

/// GET endpoint to return an SVG counter image.
/// Each time the image is requested, the counter is incremented.
/// Query parameters allow for customization (label, color, style...).
#[get("/counter/<name>/svg?<options..>")]
async fn svg_counter(
    name: &str,
    options: Option<SvgOptions>,
    counters: &State<PersistentCounterMap>,
) -> Result<SvgResponse, Status> {
    // Load the base CSS from assets/style.css.
    let base_css = include_str!("../assets/style.css");

    // Build custom CSS if parameters are provided.
    let custom_css = build_custom_css(options.clone());

    // Combine the base CSS with the custom CSS.
    let css = format!("{}\n{}", base_css, custom_css);

    let label = options
        .as_ref()
        .and_then(|opts| opts.label.clone())
        .unwrap_or_else(|| "Visits".to_string());

    // Increment the counter
    let count = counters.increment(name);

    // Get width and height
    let width = options.clone().unwrap_or_default().width.unwrap_or(150);
    let height = options.clone().unwrap_or_default().height.unwrap_or(20);

    // Generate the SVG
    let svg = svg_generator::generate_svg(&label, count, &css, width, height);

    // Build a response with  caching headers not to store the response
    let response = Response::build()
        .header(ContentType::new("image", "svg+xml"))
        .raw_header("Cache-Control", "max-age=0, no-cache, no-store, must-revalidate")
        .raw_header("Pragma", "no-cache")
        .raw_header("Expires", "0")
        .sized_body(svg.len(), Cursor::new(svg))
        .finalize();

    Ok(SvgResponse(response))
}

#[launch]
fn rocket() -> _ {
    init_env();
    rocket::build()
        .manage(PersistentCounterMap::new("/data/counters.json"))
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
