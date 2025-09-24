// frontend_visit_counter/src/app.rs
use yew::prelude::*;
use gloo::timers::callback::Timeout;

use crate::components::*;
use crate::services::ApiService;
use crate::types::*;

pub enum AppMsg {
    SetRoute(AppRoute),
    ToggleTheme,
    SetAuthToken(Option<String>),
    ShowAlert(String, String), // message, type
    HideAlert,
    Login(String), // password
    LoginSuccess(String), // token
    LoginError(String),
    Logout,
}

pub struct App {
    state: AppState,
    alert_message: Option<(String, String)>, // (message, type)
    alert_timeout: Option<Timeout>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let theme = ApiService::get_theme();
        let auth_token = ApiService::get_auth_token();
        let is_authenticated = auth_token.is_some();

        // Apply initial theme
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", theme.as_str());
            }
        }

        Self {
            state: AppState {
                theme,
                auth_token,
                is_authenticated,
                ..Default::default()
            },
            alert_message: None,
            alert_timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SetRoute(route) => {
                self.state.current_route = route;
                true
            }
            AppMsg::ToggleTheme => {
                self.state.theme = self.state.theme.toggle();
                ApiService::set_theme(&self.state.theme);

                // Apply theme to document
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-theme", self.state.theme.as_str());
                    }
                }
                true
            }
            AppMsg::SetAuthToken(token) => {
                self.state.auth_token = token.clone();
                self.state.is_authenticated = token.is_some();

                match token {
                    Some(t) => ApiService::set_auth_token(t),
                    None => ApiService::remove_auth_token(),
                }
                true
            }
            AppMsg::ShowAlert(message, alert_type) => {
                // Cancel existing timeout
                if let Some(timeout) = self.alert_timeout.take() {
                    timeout.cancel();
                }

                self.alert_message = Some((message, alert_type));

                // Set new timeout to hide alert
                let link = ctx.link().clone();
                self.alert_timeout = Some(Timeout::new(5000, move || {
                    link.send_message(AppMsg::HideAlert);
                }));
                true
            }
            AppMsg::HideAlert => {
                self.alert_message = None;
                if let Some(timeout) = self.alert_timeout.take() {
                    timeout.cancel();
                }
                true
            }
            AppMsg::Login(password) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match ApiService::login(password).await {
                        Ok(token) => link.send_message(AppMsg::LoginSuccess(token)),
                        Err(error) => link.send_message(AppMsg::LoginError(error)),
                    }
                });
                false
            }
            AppMsg::LoginSuccess(token) => {
                ctx.link().send_message(AppMsg::SetAuthToken(Some(token)));
                ctx.link().send_message(AppMsg::ShowAlert("Login successful!".to_string(), "success".to_string()));
                true
            }
            AppMsg::LoginError(error) => {
                ctx.link().send_message(AppMsg::ShowAlert(format!("Login failed: {}", error), "error".to_string()));
                false
            }
            AppMsg::Logout => {
                ctx.link().send_message(AppMsg::SetAuthToken(None));
                ctx.link().send_message(AppMsg::ShowAlert("Logged out successfully".to_string(), "info".to_string()));
                ctx.link().send_message(AppMsg::SetRoute(AppRoute::Home));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_route_change = ctx.link().callback(AppMsg::SetRoute);
        let on_theme_toggle = ctx.link().callback(|_| AppMsg::ToggleTheme);
        let on_login = ctx.link().callback(AppMsg::Login);
        let on_logout = ctx.link().callback(|_| AppMsg::Logout);
        let on_hide_alert = ctx.link().callback(|_| AppMsg::HideAlert);

        html! {
            <div class="app">
                <header class="header">
                    <div class="container">
                        <h1><i class="fas fa-chart-line"></i> { " Visit Counter" }</h1>
                        <p>{ "Generate dynamic badges and manage your visit counters" }</p>
                    </div>
                </header>

                <Navigation
                    current_route={self.state.current_route.clone()}
                    theme={self.state.theme.clone()}
                    is_authenticated={self.state.is_authenticated}
                    on_route_change={on_route_change}
                    on_theme_toggle={on_theme_toggle}
                    on_login={on_login.clone()}
                    on_logout={on_logout}
                />

                <main class="main">
                    <div class="container">
                        // Show alert if present
                        if let Some((message, alert_type)) = &self.alert_message {
                            <Alert message={message.clone()} alert_type={alert_type.clone()} on_close={on_hide_alert} />
                        }

                        // Main content based on current route
                        {
                            match self.state.current_route {
                                AppRoute::Home => html! {
                                    <BadgeGenerator />
                                },
                                AppRoute::Admin => html! {
                                    <AdminPanel
                                        auth_token={self.state.auth_token.clone()}
                                        is_authenticated={self.state.is_authenticated}
                                    />
                                },
                            }
                        }
                    </div>
                </main>
            </div>
        }
    }
}