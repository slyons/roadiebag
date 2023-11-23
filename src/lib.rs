use cfg_if::cfg_if;
pub mod app;
pub mod auth;
pub mod db;
pub mod error_template;
//pub mod fileserv;
pub mod bag;
pub(crate) mod common;
pub mod errors;
pub mod fallback;
pub mod service;
pub mod state;
mod telemetry;
mod tests;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;
    use std::panic;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        //console_error_panic_hook::set_once();
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        leptos::mount_to_body(App);
    }
}}
