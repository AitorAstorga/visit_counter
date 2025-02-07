use rocket::serde::{Deserialize, Serialize};
use rocket::form::FromForm;
use rocket::{Request, Response};
use rocket::response::{Responder, Result as RocketResult};

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

pub struct SvgResponse(
    pub Response<'static>
);

impl<'r> Responder<'r, 'static> for SvgResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> RocketResult<'static> {
        Ok(self.0)
    }
}

/// Verify that the "x-api-key" header matches the API_KEY environment variable.
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