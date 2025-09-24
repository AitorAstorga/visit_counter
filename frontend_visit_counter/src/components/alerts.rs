// frontend_visit_counter/src/components/alerts.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub message: String,
    pub alert_type: String, // "success", "error", "info"
    pub on_close: Callback<()>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let onclick = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class={classes!("alert", format!("alert-{}", props.alert_type))}>
            { &props.message }
            <button class="close-btn" onclick={onclick} style="float: right; margin-left: 1rem;">
                { "×" }
            </button>
        </div>
    }
}