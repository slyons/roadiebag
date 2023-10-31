use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/*use sqlx::Row;
pub trait FromRowPrefix<'r, R>: Sized
    where
        R: Row,
{
    // Required method
    fn from_row_prefix(row: &'r R, prefix: String) -> Result<Self, sqlx::Error>;
}*/

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;


        pub fn db_pool() -> Result<SqlitePool, ServerFnError> {
           use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }



    }
}