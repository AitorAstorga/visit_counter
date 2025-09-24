// frontend_visit_counter/src/main.rs
mod app;
mod components;
mod services;
mod types;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}