// frontend_visit_counter/src/components/mod.rs
pub mod badge_generator;
pub mod admin_panel;
pub mod navigation;
pub mod modals;
pub mod alerts;

pub use badge_generator::BadgeGenerator;
pub use admin_panel::AdminPanel;
pub use navigation::Navigation;
pub use modals::{LoginModal, CreateBadgeModal};
pub use alerts::Alert;