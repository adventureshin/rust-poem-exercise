use poem_openapi::Tags;

mod auth_controller;
mod user_controller;
mod chef_controller;

pub use auth_controller::AuthController;
pub use user_controller::UserController;
pub use chef_controller::ChefController;

#[derive(Tags)]
pub enum Tag {
    /// Authorization methods.
    Auth,
    /// User methods.
    User,
    /// Chef methods.
    Chef,
}
