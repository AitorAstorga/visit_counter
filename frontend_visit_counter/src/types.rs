// frontend_visit_counter/src/types.rs
use serde::{Deserialize, Serialize};

// Badge response types (copie of the Badge in backend models)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeResponse {
    pub name: String,
    pub count: u32,
    pub created_at: String,
    pub last_accessed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CounterResponse {
    pub name: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeListResponse {
    pub total: usize,
    pub badges: Vec<BadgeResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppRoute {
    Home,
    Admin,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub current_route: AppRoute,
    pub theme: Theme,
    pub auth_token: Option<String>,
    pub is_authenticated: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_route: AppRoute::Home,
            theme: Theme::default(),
            auth_token: None,
            is_authenticated: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BadgeConfig {
    pub name: String,
    pub label: String,
    pub style: String,
    pub width: u32,
    pub height: u32,
}

impl Default for BadgeConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            label: "Visits".to_string(),
            style: String::new(),
            width: 150,
            height: 20,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeCreateRequest {
    pub name: String,
    pub count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CounterSetRequest {
    pub count: u32,
}