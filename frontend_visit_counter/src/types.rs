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
    pub font_family: String,
    pub font_size: u32,
    pub font_weight: String,
    pub text_color: String,
    pub background_color: String,
    pub label_color: String,
    pub counter_color: String,
    pub background_label: String,
    pub background_counter: String,
    pub border_width: u32,
    pub border_color: String,
    pub border_radius: u32,
    pub logo_url: String,
    pub logo_width: u32,
    pub element_positions: String, // "label,logo,counter" format
}

impl Default for BadgeConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            label: "Visits".to_string(),
            style: String::new(),
            width: 150,
            height: 20,
            font_family: "Comfortaa, Metrophobic, sans-serif".to_string(),
            font_size: 11,
            font_weight: "normal".to_string(),
            text_color: "#ffffff".to_string(),
            background_color: "#18181b".to_string(),
            label_color: "#ffffff".to_string(),
            counter_color: "#ffffff".to_string(),
            background_label: "#18181b".to_string(),
            background_counter: "#DC26B6".to_string(),
            border_width: 0,
            border_color: "#cccccc".to_string(),
            border_radius: 3,
            logo_url: String::new(),
            logo_width: 30,
            element_positions: "label,logo,counter".to_string(),
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