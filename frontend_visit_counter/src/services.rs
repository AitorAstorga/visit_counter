// frontend_visit_counter/src/services.rs
use gloo::storage::{LocalStorage, Storage};
use gloo_net::http::Request;

use crate::types::*;

const API_BASE: &str = "/api";
const AUTH_BASE: &str = "/api/auth";
const TOKEN_KEY: &str = "authToken";
const THEME_KEY: &str = "theme";

pub struct ApiService;

impl ApiService {
    pub fn get_auth_token() -> Option<String> {
        LocalStorage::get(TOKEN_KEY).ok()
    }

    pub fn set_auth_token(token: String) {
        LocalStorage::set(TOKEN_KEY, token).ok();
    }

    pub fn remove_auth_token() {
        LocalStorage::delete(TOKEN_KEY);
    }

    pub fn get_theme() -> Theme {
        LocalStorage::get(THEME_KEY)
            .unwrap_or_else(|_| "dark".to_string())
            .as_str()
            .into()
    }

    pub fn set_theme(theme: &Theme) {
        LocalStorage::set(THEME_KEY, theme.as_str()).ok();
    }

    pub async fn login(password: String) -> Result<String, String> {
        let request = LoginRequest { password };

        let response = Request::post(&format!("{}/login", AUTH_BASE))
            .json(&request)
            .map_err(|e| format!("Network error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let token_response: TokenResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(token_response.token)
        } else {
            Err("Invalid credentials".to_string())
        }
    }

    pub async fn fetch_badges(token: &str) -> Result<BadgeListResponse, String> {
        let response = Request::get(&format!("{}/admin/badges", API_BASE))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err("Failed to fetch badges".to_string())
        }
    }

    pub async fn create_badge(token: &str, name: String, count: Option<u32>) -> Result<BadgeResponse, String> {
        let request = BadgeCreateRequest { name, count };

        let response = Request::post(&format!("{}/admin/badges", API_BASE))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)
            .map_err(|e| format!("Network error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else if response.status() == 409 {
            Err("Badge already exists".to_string())
        } else {
            Err("Failed to create badge".to_string())
        }
    }

    pub async fn update_badge(token: &str, name: String, count: u32) -> Result<BadgeResponse, String> {
        let request = CounterSetRequest { count };

        let response = Request::put(&format!("{}/admin/badges/{}", API_BASE, name))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)
            .map_err(|e| format!("Network error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err("Failed to update badge".to_string())
        }
    }

    pub async fn delete_badge(token: &str, name: String) -> Result<(), String> {
        let response = Request::delete(&format!("{}/admin/badges/{}", API_BASE, name))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err("Failed to delete badge".to_string())
        }
    }

    pub fn build_badge_url(config: &BadgeConfig) -> String {
        let mut url = format!("/counter/{}/svg", urlencoding::encode(&config.name));
        let mut params = vec![];

        if !config.label.is_empty() && config.label != "Visits" {
            params.push(format!("label={}", urlencoding::encode(&config.label)));
        }

        if config.width != 150 {
            params.push(format!("width={}", config.width));
        }

        if config.height != 20 {
            params.push(format!("height={}", config.height));
        }

        if !config.style.is_empty() {
            params.push(format!("style={}", urlencoding::encode(&config.style)));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    pub fn build_full_badge_url(config: &BadgeConfig) -> String {
        let base_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_else(|| "".to_string());
        format!("{}{}", base_url, Self::build_badge_url(config))
    }
}

impl From<&str> for Theme {
    fn from(s: &str) -> Self {
        match s {
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }
}