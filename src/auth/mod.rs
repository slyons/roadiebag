use cfg_if::cfg_if;

pub mod api;
pub mod frontend;
pub mod model;
pub(crate) mod tests;
pub use frontend::provide_auth;
pub use model::User;

cfg_if! {
    if #[cfg(feature="ssr")] {
        pub use api::auth_session;
        pub use api::AuthSession;
    }
}
