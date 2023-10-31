use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

        #[cfg(test)]
        pub(crate) mod tests {

            use anyhow::Result;
            use sqlx::SqlitePool;
            use axum_test::{TestServer, TestServerConfig};
            use dotenvy;

            pub async fn get_test_server(pool: &SqlitePool) -> Result<TestServer> {
                dotenvy::dotenv().ok();
                use crate::service::{init_logging, load_leptos_options, get_app_state, get_router};
                init_logging().await;
                let options = load_leptos_options(None, None).await;
                let state = get_app_state(pool.clone(), options);
                let config = TestServerConfig::builder()
                    .default_content_type("application/json")
                    .save_cookies()
                    .build();
                let router = get_router(state).await;

                TestServer::new_with_config(router, config)
            }
        }
    }
}