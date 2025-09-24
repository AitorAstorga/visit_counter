// frontend_visit_counter/src/components/navigation.rs
use yew::prelude::*;
use crate::types::*;

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub current_route: AppRoute,
    pub theme: Theme,
    pub is_authenticated: bool,
    pub on_route_change: Callback<AppRoute>,
    pub on_theme_toggle: Callback<()>,
    pub on_login: Callback<String>,
    pub on_logout: Callback<()>,
}

pub enum NavigationMsg {
    ShowLoginModal,
    HideLoginModal,
}

pub struct Navigation {
    show_login_modal: bool,
}

impl Component for Navigation {
    type Message = NavigationMsg;
    type Properties = NavigationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_login_modal: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigationMsg::ShowLoginModal => {
                self.show_login_modal = true;
                true
            }
            NavigationMsg::HideLoginModal => {
                self.show_login_modal = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let on_home_click = {
            let on_route_change = props.on_route_change.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                on_route_change.emit(AppRoute::Home);
            })
        };

        let on_admin_click = {
            let on_route_change = props.on_route_change.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                on_route_change.emit(AppRoute::Admin);
            })
        };

        let on_theme_click = {
            let on_theme_toggle = props.on_theme_toggle.clone();
            Callback::from(move |_| on_theme_toggle.emit(()))
        };

        let on_login_click = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(NavigationMsg::ShowLoginModal))
        };

        let on_logout_click = {
            let on_logout = props.on_logout.clone();
            Callback::from(move |_| on_logout.emit(()))
        };

        let on_modal_close = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(NavigationMsg::HideLoginModal))
        };

        let on_login_submit = {
            let on_login = props.on_login.clone();
            let link = ctx.link().clone();
            Callback::from(move |password: String| {
                on_login.emit(password);
                link.send_message(NavigationMsg::HideLoginModal);
            })
        };

        let theme_icon = match props.theme {
            Theme::Dark => "fas fa-sun",
            Theme::Light => "fas fa-moon",
        };

        let theme_title = match props.theme {
            Theme::Dark => "Switch to light theme",
            Theme::Light => "Switch to dark theme",
        };

        html! {
            <>
                <nav class="nav">
                    <div class="container">
                        <div class="nav-links">
                            <a href="#"
                               class={classes!("nav-link", if props.current_route == AppRoute::Home { Some("active") } else { None })}
                               onclick={on_home_click}>
                                <i class="fas fa-home"></i> { " Badge Generator" }
                            </a>
                            <a href="#"
                               class={classes!("nav-link", if props.current_route == AppRoute::Admin { Some("active") } else { None })}
                               onclick={on_admin_click}>
                                <i class="fas fa-cog"></i> { " Admin Panel" }
                            </a>
                        </div>
                        <div class="auth-section">
                            <button class="btn btn-secondary theme-toggle"
                                   title={theme_title}
                                   onclick={on_theme_click}>
                                <i class={theme_icon}></i>
                            </button>
                            {
                                if props.is_authenticated {
                                    html! {
                                        <div class="user-info">
                                            <div class="user-avatar">{ "A" }</div>
                                            <span>{ "Admin" }</span>
                                            <button class="btn btn-secondary btn-small" onclick={on_logout_click}>
                                                <i class="fas fa-sign-out-alt"></i> { " Logout" }
                                            </button>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <button class="btn btn-primary" onclick={on_login_click}>
                                            <i class="fas fa-sign-in-alt"></i> { " Login" }
                                        </button>
                                    }
                                }
                            }
                        </div>
                    </div>
                </nav>

                // Login Modal
                if self.show_login_modal {
                    <crate::components::LoginModal
                        on_close={on_modal_close}
                        on_submit={on_login_submit}
                    />
                }
            </>
        }
    }
}