use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use roadiebag::service::*;
        use dotenvy::dotenv;

        #[tokio::main]
        async fn main() {
            dotenv().ok();
            init_logging().await;
            let db_pool = get_db_pool().await;

            let options = load_leptos_options(None, None).await;
            let addr = options.site_addr.clone();
            let app_state = get_app_state(db_pool, options);
            let app = get_router(app_state).await;

            tracing::info!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    } else {
        pub fn main() {
        }
    }
}