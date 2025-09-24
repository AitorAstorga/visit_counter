// frontend_visit_counter/src/components/badge_generator.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::utils::window;

use crate::services::ApiService;
use crate::types::BadgeConfig;

pub enum BadgeGeneratorMsg {
    UpdateName(String),
    UpdateLabel(String),
    UpdateStyle(String),
    UpdateWidth(String),
    UpdateHeight(String),
    CopyCode,
}

pub struct BadgeGenerator {
    config: BadgeConfig,
    copy_success: bool,
}

impl Component for BadgeGenerator {
    type Message = BadgeGeneratorMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: BadgeConfig::default(),
            copy_success: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BadgeGeneratorMsg::UpdateName(name) => {
                // Sanitize badge name (only letters, numbers, hyphens, underscores)
                let sanitized = name.chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                    .collect();
                self.config.name = sanitized;
                true
            }
            BadgeGeneratorMsg::UpdateLabel(label) => {
                self.config.label = label;
                true
            }
            BadgeGeneratorMsg::UpdateStyle(style) => {
                self.config.style = style;
                true
            }
            BadgeGeneratorMsg::UpdateWidth(width) => {
                if let Ok(w) = width.parse::<u32>() {
                    self.config.width = w;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::UpdateHeight(height) => {
                if let Ok(h) = height.parse::<u32>() {
                    self.config.height = h;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::CopyCode => {
                let full_url = ApiService::build_full_badge_url(&self.config);
                let html_code = format!("<img src=\"{}\" alt=\"Visit Counter\" />", full_url);

                let clipboard = window().navigator().clipboard();
                let _ = clipboard.write_text(&html_code);
                self.copy_success = true;

                // Reset copy success after 2 seconds
                let timeout = gloo::timers::callback::Timeout::new(2000, || {});
                timeout.forget();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let badge_url = if self.config.name.is_empty() {
            ApiService::build_badge_url(&BadgeConfig {
                name: "example".to_string(),
                ..self.config.clone()
            })
        } else {
            ApiService::build_badge_url(&self.config)
        };

        let full_url = if self.config.name.is_empty() {
            ApiService::build_full_badge_url(&BadgeConfig {
                name: "example".to_string(),
                ..self.config.clone()
            })
        } else {
            ApiService::build_full_badge_url(&self.config)
        };

        let html_code = format!("<img src=\"{}\" alt=\"Visit Counter\" />", full_url);

        // Event handlers
        let on_name_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateName(input.value()));
                }
            })
        };

        let on_label_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateLabel(input.value()));
                }
            })
        };

        let on_style_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(textarea) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateStyle(textarea.value()));
                }
            })
        };

        let on_width_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateWidth(input.value()));
                }
            })
        };

        let on_height_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateHeight(input.value()));
                }
            })
        };

        let on_copy_click = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(BadgeGeneratorMsg::CopyCode);
            })
        };

        html! {
            <div id="home-section" class="section active">
                <div class="card">
                    <h2><i class="fas fa-magic"></i> { " Badge Generator" }</h2>
                    <p>{ "Create custom visit counter badges for your websites and projects." }</p>

                    <div class="form-grid">
                        <div class="form-group">
                            <label for="badge-name">{ "Badge Name" }</label>
                            <input type="text"
                                   id="badge-name"
                                   placeholder="my-awesome-project"
                                   class="form-input"
                                   value={self.config.name.clone()}
                                   oninput={on_name_input} />
                            <small>{ "Use only letters, numbers, hyphens, and underscores" }</small>
                        </div>

                        <div class="form-group">
                            <label for="badge-label">{ "Label Text" }</label>
                            <input type="text"
                                   id="badge-label"
                                   placeholder="Visits"
                                   class="form-input"
                                   value={self.config.label.clone()}
                                   oninput={on_label_input} />
                        </div>

                        <div class="form-group">
                            <label for="badge-style">{ "Custom Style" }</label>
                            <textarea id="badge-style"
                                     placeholder=":root { --background-counter: #ff6b6b; }"
                                     class="form-textarea"
                                     rows="3"
                                     value={self.config.style.clone()}
                                     oninput={on_style_input}></textarea>
                            <small>{ "CSS variables to customize appearance" }</small>
                        </div>

                        <div class="form-group">
                            <label for="badge-width">{ "Width" }</label>
                            <input type="number"
                                   id="badge-width"
                                   placeholder="150"
                                   class="form-input"
                                   value={self.config.width.to_string()}
                                   oninput={on_width_input} />
                        </div>

                        <div class="form-group">
                            <label for="badge-height">{ "Height" }</label>
                            <input type="number"
                                   id="badge-height"
                                   placeholder="20"
                                   class="form-input"
                                   value={self.config.height.to_string()}
                                   oninput={on_height_input} />
                        </div>
                    </div>

                    <div class="preview-section">
                        <h3><i class="fas fa-eye"></i> { " Live Preview" }</h3>
                        <div class="preview-container">
                            <img src={badge_url} alt="Badge Preview" class="badge-preview" />
                        </div>

                        <div class="code-section">
                            <h4><i class="fas fa-code"></i> { " HTML Code" }</h4>
                            <div class="code-container">
                                <pre>{ html_code }</pre>
                                <button class="btn btn-secondary" onclick={on_copy_click}>
                                    if self.copy_success {
                                        <><i class="fas fa-check"></i> { " Copied!" }</>
                                    } else {
                                        <><i class="fas fa-copy"></i> { " Copy" }</>
                                    }
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}