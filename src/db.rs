use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;

        pub fn db_pool() -> Result<SqlitePool, ServerFnError> {
           use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

    }
}