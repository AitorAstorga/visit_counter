// frontend_visit_counter/src/components/modals.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct LoginModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<String>,
}

#[function_component(LoginModal)]
pub fn login_modal(props: &LoginModalProps) -> Html {
    let password_ref = use_node_ref();

    let on_backdrop_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::Element>() {
                if target.class_name().contains("modal") {
                    on_close.emit(());
                }
            }
        })
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let password_ref = password_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = password_ref.cast::<HtmlInputElement>() {
                let password = input.value();
                if !password.is_empty() {
                    on_submit.emit(password);
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <div class="modal show" onclick={on_backdrop_click}>
            <div class="modal-content">
                <div class="modal-header">
                    <h3><i class="fas fa-key"></i> { " Admin Login" }</h3>
                    <button class="close-btn" onclick={on_close_click}>{ "×" }</button>
                </div>
                <form class="modal-body" onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="admin-password">{ "Password" }</label>
                        <input type="password"
                               id="admin-password"
                               class="form-input"
                               ref={password_ref}
                               required=true />
                    </div>
                    <div class="modal-footer">
                        <button type="submit" class="btn btn-primary">{ "Login" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CreateBadgeModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<(String, Option<u32>)>,
}

#[function_component(CreateBadgeModal)]
pub fn create_badge_modal(props: &CreateBadgeModalProps) -> Html {
    let name_ref = use_node_ref();
    let count_ref = use_node_ref();

    let on_backdrop_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<web_sys::Element>() {
                if target.class_name().contains("modal") {
                    on_close.emit(());
                }
            }
        })
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_submit = {
        let on_submit = props.on_submit.clone();
        let name_ref = name_ref.clone();
        let count_ref = count_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(name_input) = name_ref.cast::<HtmlInputElement>() {
                let name = name_input.value().trim().to_string();
                if !name.is_empty() {
                    let count = count_ref.cast::<HtmlInputElement>()
                        .and_then(|input| {
                            let value = input.value();
                            if value.is_empty() {
                                None
                            } else {
                                value.parse::<u32>().ok()
                            }
                        });

                    on_submit.emit((name, count));
                    name_input.set_value("");
                    if let Some(count_input) = count_ref.cast::<HtmlInputElement>() {
                        count_input.set_value("");
                    }
                }
            }
        })
    };

    html! {
        <div class="modal show" onclick={on_backdrop_click}>
            <div class="modal-content">
                <div class="modal-header">
                    <h3><i class="fas fa-plus"></i> { " Create New Badge" }</h3>
                    <button class="close-btn" onclick={on_close_click}>{ "×" }</button>
                </div>
                <form class="modal-body" onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="new-badge-name">{ "Badge Name" }</label>
                        <input type="text"
                               id="new-badge-name"
                               class="form-input"
                               ref={name_ref}
                               required=true />
                    </div>
                    <div class="form-group">
                        <label for="new-badge-count">{ "Initial Count (optional)" }</label>
                        <input type="number"
                               id="new-badge-count"
                               class="form-input"
                               ref={count_ref}
                               min="0" />
                    </div>
                    <div class="modal-footer">
                        <button type="submit" class="btn btn-primary">{ "Create Badge" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}