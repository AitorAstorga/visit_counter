// backend_visit_counter/src/main.rs
#[macro_use]
extern crate rocket;

mod models;
mod persistent_counter;
mod svg_generator;

use std::io::Cursor;

use models::{ApiKey, CounterResponse, CounterSetRequest, SvgOptions, SvgResponse,
           BadgeCreateRequest, BadgeResponse, BadgeListResponse};
use persistent_counter::PersistentCounterMap;

use rocket::http::{ContentType, Status, Method};
use rocket::serde::json::Json;
use rocket::{Response, State, fs::FileServer};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use svg_generator::build_custom_css;

// Optionally load environment variables from .env.
fn init_env() {
    dotenv::dotenv().ok();
}

/// GET endpoint to return a counter as JSON (without incrementing)
#[get("/counter/<name>")]
async fn get_counter_json(name: &str, counters: &State<PersistentCounterMap>) -> Json<CounterResponse> {
    let count = counters.get(name);
    Json(CounterResponse {
        name: name.to_string(),
        count,
    })
}

/// POST endpoint to increment a counter (returns the new count)
#[post("/counter/<name>/increment")]
async fn increment_counter_json(name: &str, counters: &State<PersistentCounterMap>) -> Json<CounterResponse> {
    let count = counters.increment(name);
    Json(CounterResponse {
        name: name.to_string(),
        count,
    })
}

/// PUT endpoint to set a counter to a given value (for administration)
/// The caller must include a valid API key in the "x-api-key" header.
#[put("/counter/<name>", data = "<new_value>")]
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
    let base_css = include_str!("../../assets/style.css");

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
    let svg = svg_generator::generate_svg(&label, count, &css, width, height, options.as_ref());

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

/// Authentication endpoints using prisma_auth
#[post("/login", format = "json", data = "<body>")]
fn login(body: Json<prisma_auth::LoginRequest>, store: &State<prisma_auth::backend::TokenStore>) -> Result<Json<prisma_auth::TokenResponse>, Status> {
    prisma_auth::backend::login_handler(body, store)
}

/// Admin endpoint to list all badges
#[get("/badges")]
async fn admin_list_badges(
    _auth: prisma_auth::backend::AuthGuard,
    counters: &State<PersistentCounterMap>,
) -> Json<BadgeListResponse> {
    let badges = counters.get_all_badges();
    Json(BadgeListResponse {
        total: badges.len(),
        badges,
    })
}

/// Admin endpoint to get a specific badge
#[get("/badges/<name>")]
async fn admin_get_badge(
    name: &str,
    _auth: prisma_auth::backend::AuthGuard,
    counters: &State<PersistentCounterMap>,
) -> Result<Json<BadgeResponse>, Status> {
    match counters.get_badge(name) {
        Some(badge) => Ok(Json(badge)),
        None => Err(Status::NotFound),
    }
}

/// Admin endpoint to create a new badge
#[post("/badges", format = "json", data = "<request>")]
async fn admin_create_badge(
    request: Json<BadgeCreateRequest>,
    _auth: prisma_auth::backend::AuthGuard,
    counters: &State<PersistentCounterMap>,
) -> Result<Json<BadgeResponse>, Status> {
    // Check if badge already exists
    if counters.get_badge(&request.name).is_some() {
        return Err(Status::Conflict);
    }

    let badge = counters.create_badge(&request.name, request.count);
    Ok(Json(badge))
}

/// Admin endpoint to update a badge's counter
#[put("/badges/<name>", format = "json", data = "<request>")]
async fn admin_update_badge(
    name: &str,
    request: Json<CounterSetRequest>,
    _auth: prisma_auth::backend::AuthGuard,
    counters: &State<PersistentCounterMap>,
) -> Result<Json<BadgeResponse>, Status> {
    // Check if badge exists
    if counters.get_badge(name).is_none() {
        return Err(Status::NotFound);
    }

    counters.set(name, request.count);

    match counters.get_badge(name) {
        Some(badge) => Ok(Json(badge)),
        None => Err(Status::InternalServerError),
    }
}

/// Admin endpoint to delete a badge
#[delete("/badges/<name>")]
async fn admin_delete_badge(
    name: &str,
    _auth: prisma_auth::backend::AuthGuard,
    counters: &State<PersistentCounterMap>,
) -> Status {
    if counters.delete_badge(name) {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

#[launch]
fn rocket() -> _ {
    init_env();

    // Initialize token store for authentication
    let token_store = prisma_auth::backend::TokenStore::new();

    // Configure CORS
    let allowed_origins = AllowedOrigins::all();
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type", "x-api-key"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error configuring CORS");

    rocket::build()
        .attach(cors)
        .manage(PersistentCounterMap::new("/data/counters.json"))
        .manage(token_store)
        .mount("/", FileServer::from(
            if std::path::Path::new("/app/frontend").exists() {
                "/app/frontend"  // Production Docker path
            } else {
                "../frontend_visit_counter/dist"  // Development path
            }
        ))
        .mount("/api/auth", routes![login])
        .mount("/", routes![svg_counter])
        .mount("/api", routes![
            get_counter_json,
            increment_counter_json,
            set_counter_json
        ])
        .mount("/api/admin", routes![
            admin_list_badges,
            admin_get_badge,
            admin_create_badge,
            admin_update_badge,
            admin_delete_badge
        ])
}