#![allow(dead_code)]
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use axum::{
            Router,
        };
        use roadiebag::app::App;
        use roadiebag::service::*;

        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use leptos::{logging::log, provide_context, get_configuration};
        use dotenvy::dotenv;
        use std::env;

        #[tokio::main]
        async fn main() {
            dotenv().ok();
            init_logging().await;
            let db_pool = get_db_pool().await;

            let options = load_leptos_options(None, None).await;
            let addr = options.site_addr.clone();
            let app_state = get_app_state(db_pool, options);
            log!("Routes are {:?}", app_state.routes);
            let app = get_router(app_state).await;

            log!("listening on http://{}", &addr);
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