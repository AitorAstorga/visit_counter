// frontend_visit_counter/src/components/admin_panel.rs
use yew::prelude::*;

use crate::components::CreateBadgeModal;
use crate::services::ApiService;
use crate::types::*;

#[derive(Properties, PartialEq)]
pub struct AdminPanelProps {
    pub auth_token: Option<String>,
    pub is_authenticated: bool,
}

pub enum AdminPanelMsg {
    LoadBadges,
    BadgesLoaded(BadgeListResponse),
    LoadError(String),
    ShowCreateModal,
    HideCreateModal,
    CreateBadge(String, Option<u32>),
    BadgeCreated(BadgeResponse),
    CreateError(String),
    EditBadge(String, u32),
    UpdateBadge(String, u32),
    BadgeUpdated(BadgeResponse),
    UpdateError(String),
    DeleteBadge(String),
    BadgeDeleted(String),
    DeleteError(String),
}

pub struct AdminPanel {
    badges: Vec<BadgeResponse>,
    loading: bool,
    error: Option<String>,
    show_create_modal: bool,
}

impl Component for AdminPanel {
    type Message = AdminPanelMsg;
    type Properties = AdminPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Load badges if authenticated
        if ctx.props().is_authenticated {
            ctx.link().send_message(AdminPanelMsg::LoadBadges);
        }

        Self {
            badges: Vec::new(),
            loading: false,
            error: None,
            show_create_modal: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Reload badges when authentication changes
        if ctx.props().is_authenticated {
            ctx.link().send_message(AdminPanelMsg::LoadBadges);
        } else {
            self.badges.clear();
            self.error = None;
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AdminPanelMsg::LoadBadges => {
                if let Some(token) = &ctx.props().auth_token {
                    self.loading = true;
                    self.error = None;

                    let token = token.clone();
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match ApiService::fetch_badges(&token).await {
                            Ok(response) => link.send_message(AdminPanelMsg::BadgesLoaded(response)),
                            Err(error) => link.send_message(AdminPanelMsg::LoadError(error)),
                        }
                    });
                }
                true
            }
            AdminPanelMsg::BadgesLoaded(response) => {
                self.badges = response.badges;
                self.loading = false;
                true
            }
            AdminPanelMsg::LoadError(error) => {
                self.error = Some(error);
                self.loading = false;
                true
            }
            AdminPanelMsg::ShowCreateModal => {
                self.show_create_modal = true;
                true
            }
            AdminPanelMsg::HideCreateModal => {
                self.show_create_modal = false;
                true
            }
            AdminPanelMsg::CreateBadge(name, count) => {
                if let Some(token) = &ctx.props().auth_token {
                    let token = token.clone();
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match ApiService::create_badge(&token, name, count).await {
                            Ok(badge) => link.send_message(AdminPanelMsg::BadgeCreated(badge)),
                            Err(error) => link.send_message(AdminPanelMsg::CreateError(error)),
                        }
                    });
                }
                false
            }
            AdminPanelMsg::BadgeCreated(badge) => {
                self.badges.push(badge);
                self.show_create_modal = false;
                true
            }
            AdminPanelMsg::CreateError(error) => {
                self.error = Some(error);
                false
            }
            AdminPanelMsg::EditBadge(name, _current_count) => {
                if let Some(new_count_str) = web_sys::window()
                    .and_then(|w| w.prompt_with_message(&format!("Enter new count for \"{}\":", name)).ok())
                    .flatten()
                {
                    if let Ok(new_count) = new_count_str.parse::<u32>() {
                        ctx.link().send_message(AdminPanelMsg::UpdateBadge(name, new_count));
                    }
                }
                false
            }
            AdminPanelMsg::UpdateBadge(name, count) => {
                if let Some(token) = &ctx.props().auth_token {
                    let token = token.clone();
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match ApiService::update_badge(&token, name, count).await {
                            Ok(badge) => link.send_message(AdminPanelMsg::BadgeUpdated(badge)),
                            Err(error) => link.send_message(AdminPanelMsg::UpdateError(error)),
                        }
                    });
                }
                false
            }
            AdminPanelMsg::BadgeUpdated(updated_badge) => {
                if let Some(index) = self.badges.iter().position(|b| b.name == updated_badge.name) {
                    self.badges[index] = updated_badge;
                }
                true
            }
            AdminPanelMsg::UpdateError(error) => {
                self.error = Some(error);
                false
            }
            AdminPanelMsg::DeleteBadge(name) => {
                if web_sys::window()
                    .and_then(|w| w.confirm_with_message(&format!("Are you sure you want to delete the badge \"{}\"?", name)).ok())
                    .unwrap_or(false)
                {
                    if let Some(token) = &ctx.props().auth_token {
                        let token = token.clone();
                        let name_clone = name.clone();
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match ApiService::delete_badge(&token, name_clone.clone()).await {
                                Ok(()) => link.send_message(AdminPanelMsg::BadgeDeleted(name_clone)),
                                Err(error) => link.send_message(AdminPanelMsg::DeleteError(error)),
                            }
                        });
                    }
                }
                false
            }
            AdminPanelMsg::BadgeDeleted(name) => {
                self.badges.retain(|b| b.name != name);
                true
            }
            AdminPanelMsg::DeleteError(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().is_authenticated {
            return html! {
                <div id="admin-section" class="section active">
                    <div class="card">
                        <div class="card-header">
                            <h2><i class="fas fa-shield-alt"></i> { " Admin Panel" }</h2>
                        </div>
                        <div class="admin-content">
                            <p class="login-prompt">{ "Please log in to access the admin panel." }</p>
                        </div>
                    </div>
                </div>
            };
        }

        let on_refresh = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(AdminPanelMsg::LoadBadges))
        };

        let on_create_badge = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(AdminPanelMsg::ShowCreateModal))
        };

        let on_modal_close = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(AdminPanelMsg::HideCreateModal))
        };

        let on_modal_submit = {
            let link = ctx.link().clone();
            Callback::from(move |(name, count): (String, Option<u32>)| {
                link.send_message(AdminPanelMsg::CreateBadge(name, count))
            })
        };

        // Calculate stats
        let total_badges = self.badges.len();
        let total_visits: u32 = self.badges.iter().map(|b| b.count).sum();
        let recent_activity = self.badges.len(); // Simplified for now

        html! {
            <>
                <div id="admin-section" class="section active">
                    <div class="card">
                        <div class="card-header">
                            <h2><i class="fas fa-shield-alt"></i> { " Admin Panel" }</h2>
                            <button class="btn btn-secondary" onclick={on_refresh}>
                                <i class="fas fa-sync-alt"></i> { " Refresh" }
                            </button>
                        </div>

                        <div class="admin-content">
                            if self.loading {
                                <div style="text-align: center; padding: 2rem;">
                                    <div class="loading"></div>
                                </div>
                            } else if let Some(error) = &self.error {
                                <p style="text-align: center; padding: 2rem; color: #ef4444;">
                                    { format!("Error: {}", error) }
                                </p>
                            } else {
                                <>
                                    <div class="stats-grid">
                                        <div class="stat-card">
                                            <div class="stat-number">{ total_badges }</div>
                                            <div class="stat-label">{ "Total Badges" }</div>
                                        </div>
                                        <div class="stat-card">
                                            <div class="stat-number">{ total_visits }</div>
                                            <div class="stat-label">{ "Total Visits" }</div>
                                        </div>
                                        <div class="stat-card">
                                            <div class="stat-number">{ recent_activity }</div>
                                            <div class="stat-label">{ "Active (24h)" }</div>
                                        </div>
                                    </div>

                                    <div class="card-header">
                                        <h3><i class="fas fa-list"></i> { " Badge Management" }</h3>
                                        <button class="btn btn-primary" onclick={on_create_badge}>
                                            <i class="fas fa-plus"></i> { " Create Badge" }
                                        </button>
                                    </div>

                                    <div class="badge-list">
                                        if self.badges.is_empty() {
                                            <p class="text-center" style="padding: 2rem; color: var(--text-secondary);">
                                                { "No badges found. Create your first badge!" }
                                            </p>
                                        } else {
                                            { for self.badges.iter().map(|badge| self.render_badge_item(ctx, badge)) }
                                        }
                                    </div>
                                </>
                            }
                        </div>
                    </div>
                </div>

                if self.show_create_modal {
                    <CreateBadgeModal
                        on_close={on_modal_close}
                        on_submit={on_modal_submit}
                    />
                }
            </>
        }
    }
}

impl AdminPanel {
    fn render_badge_item(&self, ctx: &Context<Self>, badge: &BadgeResponse) -> Html {
        let badge_name = badge.name.clone();
        let badge_count = badge.count;

        let on_edit = {
            let link = ctx.link().clone();
            let name = badge_name.clone();
            Callback::from(move |_| {
                link.send_message(AdminPanelMsg::EditBadge(name.clone(), badge_count))
            })
        };

        let on_delete = {
            let link = ctx.link().clone();
            let name = badge_name.clone();
            Callback::from(move |_| {
                link.send_message(AdminPanelMsg::DeleteBadge(name.clone()))
            })
        };

        html! {
            <div class="badge-item">
                <div class="badge-info">
                    <div class="badge-name">{ &badge.name }</div>
                    <div class="badge-stats">
                        <span><i class="fas fa-eye"></i> { format!(" {} visits", badge.count) }</span>
                        <span><i class="fas fa-calendar"></i> { format!(" Created {}", badge.created_at) }</span>
                        <span><i class="fas fa-clock"></i> { format!(" Last accessed {}", badge.last_accessed) }</span>
                    </div>
                </div>
                <div class="badge-actions">
                    <button class="btn btn-secondary btn-small" onclick={on_edit}>
                        <i class="fas fa-edit"></i> { " Edit" }
                    </button>
                    <button class="btn btn-danger btn-small" onclick={on_delete}>
                        <i class="fas fa-trash"></i> { " Delete" }
                    </button>
                </div>
            </div>
        }
    }
}