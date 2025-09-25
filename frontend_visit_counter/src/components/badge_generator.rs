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
    UpdateFontFamily(String),
    UpdateFontSize(String),
    UpdateFontWeight(String),
    UpdateBorderWidth(String),
    UpdateBorderColor(String),
    UpdateBorderRadius(String),
    UpdateLogoUrl(String),
    UpdateLogoWidth(String),
    UpdateElementPositions(String),
    UpdateLabelColor(String),
    UpdateCounterColor(String),
    UpdateBackgroundLabel(String),
    UpdateBackgroundCounter(String),
    ToggleAdvancedPanel,
    CopyCode,
}

pub struct BadgeGenerator {
    config: BadgeConfig,
    copy_success: bool,
    advanced_panel_open: bool,
}

impl Component for BadgeGenerator {
    type Message = BadgeGeneratorMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: BadgeConfig::default(),
            copy_success: false,
            advanced_panel_open: false,
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
            BadgeGeneratorMsg::UpdateFontFamily(font_family) => {
                self.config.font_family = font_family;
                true
            }
            BadgeGeneratorMsg::UpdateFontSize(font_size) => {
                if let Ok(size) = font_size.parse::<u32>() {
                    self.config.font_size = size;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::UpdateFontWeight(font_weight) => {
                self.config.font_weight = font_weight;
                true
            }
            BadgeGeneratorMsg::UpdateBorderWidth(border_width) => {
                if let Ok(width) = border_width.parse::<u32>() {
                    self.config.border_width = width;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::UpdateBorderColor(border_color) => {
                self.config.border_color = border_color;
                true
            }
            BadgeGeneratorMsg::UpdateBorderRadius(border_radius) => {
                if let Ok(radius) = border_radius.parse::<u32>() {
                    self.config.border_radius = radius;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::UpdateLogoUrl(logo_url) => {
                self.config.logo_url = logo_url;
                true
            }
            BadgeGeneratorMsg::UpdateLogoWidth(logo_width) => {
                if let Ok(width) = logo_width.parse::<u32>() {
                    self.config.logo_width = width;
                    true
                } else {
                    false
                }
            }
            BadgeGeneratorMsg::UpdateElementPositions(positions) => {
                self.config.element_positions = positions;
                true
            }
            BadgeGeneratorMsg::UpdateLabelColor(label_color) => {
                self.config.label_color = label_color;
                true
            }
            BadgeGeneratorMsg::UpdateCounterColor(counter_color) => {
                self.config.counter_color = counter_color;
                true
            }
            BadgeGeneratorMsg::UpdateBackgroundLabel(background_label) => {
                self.config.background_label = background_label;
                true
            }
            BadgeGeneratorMsg::UpdateBackgroundCounter(background_counter) => {
                self.config.background_counter = background_counter;
                true
            }
            BadgeGeneratorMsg::ToggleAdvancedPanel => {
                self.advanced_panel_open = !self.advanced_panel_open;
                true
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

        let on_advanced_toggle = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(BadgeGeneratorMsg::ToggleAdvancedPanel);
            })
        };

        // Advanced customization event handlers
        let on_font_family_change = {
            let link = ctx.link().clone();
            Callback::from(move |e: yew::events::Event| {
                if let Some(select) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateFontFamily(select.value()));
                }
            })
        };

        let on_font_size_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateFontSize(input.value()));
                }
            })
        };

        let on_font_weight_change = {
            let link = ctx.link().clone();
            Callback::from(move |e: yew::events::Event| {
                if let Some(select) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateFontWeight(select.value()));
                }
            })
        };


        let on_border_width_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateBorderWidth(input.value()));
                }
            })
        };

        let on_border_color_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateBorderColor(input.value()));
                }
            })
        };

        let on_border_radius_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateBorderRadius(input.value()));
                }
            })
        };


        let on_logo_url_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateLogoUrl(input.value()));
                }
            })
        };

        let on_label_color_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateLabelColor(input.value()));
                }
            })
        };

        let on_counter_color_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateCounterColor(input.value()));
                }
            })
        };

        let on_background_label_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateBackgroundLabel(input.value()));
                }
            })
        };

        let on_background_counter_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateBackgroundCounter(input.value()));
                }
            })
        };

        let on_logo_width_input = {
            let link = ctx.link().clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateLogoWidth(input.value()));
                }
            })
        };

        let on_element_positions_change = {
            let link = ctx.link().clone();
            Callback::from(move |e: yew::events::Event| {
                if let Some(select) = e.target_dyn_into::<HtmlInputElement>() {
                    link.send_message(BadgeGeneratorMsg::UpdateElementPositions(select.value()));
                }
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

                        // Advanced customization toggle in form grid
                        <div class="form-group form-group-full">
                            <button class="btn btn-outline advanced-toggle" onclick={on_advanced_toggle}>
                                <span class={format!("toggle-icon {}", if self.advanced_panel_open { "rotated" } else { "" })}>
                                    { "❖" }
                                </span>
                                { " Advanced Customization" }
                            </button>
                        </div>
                    </div>

                    if self.advanced_panel_open {
                        <div class="advanced-panel">
                            <div class="form-grid">
                                // Font customization
                                <div class="form-group">
                                    <label for="font-family">{ "Font Family" }</label>
                                    <select id="font-family"
                                            class="form-input"
                                            onchange={on_font_family_change}
                                            value={self.config.font_family.clone()}>
                                        <option value="Comfortaa, Metrophobic, sans-serif">{ "Comfortaa (Default)" }</option>
                                        <option value="Arial, sans-serif">{ "Arial" }</option>
                                        <option value="Helvetica, Arial, sans-serif">{ "Helvetica" }</option>
                                        <option value="Georgia, serif">{ "Georgia" }</option>
                                        <option value="Times New Roman, serif">{ "Times New Roman" }</option>
                                        <option value="Courier New, monospace">{ "Courier New" }</option>
                                        <option value="Verdana, sans-serif">{ "Verdana" }</option>
                                        <option value="Trebuchet MS, sans-serif">{ "Trebuchet MS" }</option>
                                        <option value="Impact, sans-serif">{ "Impact" }</option>
                                        <option value="Comic Sans MS, cursive">{ "Comic Sans MS" }</option>
                                    </select>
                                </div>

                                <div class="form-group">
                                    <label for="font-size">{ "Font Size (px)" }</label>
                                    <input type="number"
                                           id="font-size"
                                           placeholder="12"
                                           class="form-input"
                                           value={self.config.font_size.to_string()}
                                           oninput={on_font_size_input} />
                                </div>

                                <div class="form-group">
                                    <label for="font-weight">{ "Font Weight" }</label>
                                    <select id="font-weight"
                                            class="form-input"
                                            onchange={on_font_weight_change}
                                            value={self.config.font_weight.clone()}>
                                        <option value="normal">{ "Normal" }</option>
                                        <option value="bold">{ "Bold" }</option>
                                        <option value="lighter">{ "Lighter" }</option>
                                        <option value="100">{ "100 - Thin" }</option>
                                        <option value="300">{ "300 - Light" }</option>
                                        <option value="400">{ "400 - Normal" }</option>
                                        <option value="500">{ "500 - Medium" }</option>
                                        <option value="600">{ "600 - Semi Bold" }</option>
                                        <option value="700">{ "700 - Bold" }</option>
                                        <option value="900">{ "900 - Black" }</option>
                                    </select>
                                </div>

                                // Color customization - separated for label and counter
                                <div class="form-group">
                                    <label for="label-color">{ "Label Text Color" }</label>
                                    <div class="color-input-group">
                                        <input type="color"
                                               id="label-color"
                                               class="color-picker"
                                               value={self.config.label_color.clone()}
                                               oninput={on_label_color_input.clone()} />
                                        <input type="text"
                                               class="form-input color-text"
                                               value={self.config.label_color.clone()}
                                               placeholder="#ffffff"
                                               oninput={on_label_color_input} />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="counter-color">{ "Counter Text Color" }</label>
                                    <div class="color-input-group">
                                        <input type="color"
                                               id="counter-color"
                                               class="color-picker"
                                               value={self.config.counter_color.clone()}
                                               oninput={on_counter_color_input.clone()} />
                                        <input type="text"
                                               class="form-input color-text"
                                               value={self.config.counter_color.clone()}
                                               placeholder="#ffffff"
                                               oninput={on_counter_color_input} />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="background-label">{ "Label Background Color" }</label>
                                    <div class="color-input-group">
                                        <input type="color"
                                               id="background-label"
                                               class="color-picker"
                                               value={self.config.background_label.clone()}
                                               oninput={on_background_label_input.clone()} />
                                        <input type="text"
                                               class="form-input color-text"
                                               value={self.config.background_label.clone()}
                                               placeholder="#18181b"
                                               oninput={on_background_label_input} />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="background-counter">{ "Counter Background Color" }</label>
                                    <div class="color-input-group">
                                        <input type="color"
                                               id="background-counter"
                                               class="color-picker"
                                               value={self.config.background_counter.clone()}
                                               oninput={on_background_counter_input.clone()} />
                                        <input type="text"
                                               class="form-input color-text"
                                               value={self.config.background_counter.clone()}
                                               placeholder="#DC26B6"
                                               oninput={on_background_counter_input} />
                                    </div>
                                </div>

                                // Border customization
                                <div class="form-group">
                                    <label for="border-width">{ "Border Width (px)" }</label>
                                    <input type="number"
                                           id="border-width"
                                           placeholder="0"
                                           class="form-input"
                                           value={self.config.border_width.to_string()}
                                           oninput={on_border_width_input} />
                                </div>

                                <div class="form-group">
                                    <label for="border-color">{ "Border Color" }</label>
                                    <div class="color-input-group">
                                        <input type="color"
                                               id="border-color"
                                               class="color-picker"
                                               value={self.config.border_color.clone()}
                                               oninput={on_border_color_input.clone()} />
                                        <input type="text"
                                               class="form-input color-text"
                                               value={self.config.border_color.clone()}
                                               placeholder="#cccccc"
                                               oninput={on_border_color_input} />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="border-radius">{ "Border Radius (px)" }</label>
                                    <input type="number"
                                           id="border-radius"
                                           placeholder="3"
                                           class="form-input"
                                           value={self.config.border_radius.to_string()}
                                           oninput={on_border_radius_input} />
                                </div>


                                // Element positioning
                                <div class="form-group">
                                    <label for="element-positions">{ "Element Layout" }</label>
                                    <select id="element-positions"
                                            class="form-input"
                                            onchange={on_element_positions_change}
                                            value={self.config.element_positions.clone()}>
                                        <option value="label,logo,counter">{ "Label | Logo | Counter" }</option>
                                        <option value="logo,label,counter">{ "Logo | Label | Counter" }</option>
                                        <option value="label,counter,logo">{ "Label | Counter | Logo" }</option>
                                        <option value="label,counter">{ "Label | Counter (No Logo)" }</option>
                                        <option value="counter,label">{ "Counter | Label" }</option>
                                    </select>
                                </div>

                                // Logo embedding
                                <div class="form-group">
                                    <label for="logo-url">{ "Logo URL" }</label>
                                    <input type="url"
                                           id="logo-url"
                                           placeholder="https://example.com/logo.png"
                                           class="form-input"
                                           value={self.config.logo_url.clone()}
                                           oninput={on_logo_url_input} />
                                    <small>{ "URL to an image file (PNG, JPG, SVG)" }</small>
                                </div>

                                <div class="form-group">
                                    <label for="logo-width">{ "Logo Width (px)" }</label>
                                    <input type="number"
                                           id="logo-width"
                                           placeholder="30"
                                           class="form-input"
                                           min="10"
                                           max="100"
                                           value={self.config.logo_width.to_string()}
                                           oninput={on_logo_width_input} />
                                </div>
                            </div>
                        </div>
                    }

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