use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use axum::{
            response::{Response, IntoResponse},
            routing::get,
            extract::{Path, State, RawQuery},
            http::{Request, header::HeaderMap},
            body::Body as AxumBody,
            Router,
        };
        use crate::app::App;
        use crate::state::AppState;
        use crate::fallback::file_and_error_handler;
        use crate::auth::{AuthSession, User};
        use crate::telemetry::*;

        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use leptos::{logging::log, provide_context, get_configuration};

        use tower_http::trace::TraceLayer;
        use log::Level;
        use std::str::FromStr;
        use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthSessionLayer, AuthConfig, SessionSqlitePool};
        use dotenvy::dotenv;
        use std::env;
        use std::net::SocketAddr;
        use leptos::leptos_config::errors::LeptosConfigError;

        #[tracing::instrument(level = "info", fields(error))]
        async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession,
            path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>)
        -> impl IntoResponse {
            handle_server_fns_with_context(path, headers, raw_query, move || {
                provide_context(auth_session.clone());
                provide_context(app_state.pool.clone());
            }, request).await
        }

        #[tracing::instrument(level = "info", fields(error))]
        async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>,
            req: Request<AxumBody>) -> Response {
            let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(),
                app_state.routes.clone(),
                move || {
                    provide_context(auth_session.clone());
                    provide_context(app_state.pool.clone());
                },
                App
            );
            handler(req).await.into_response()
        }

        pub async fn init_logging() {
            /*let log_level = env::var("LOG_LEVEL")
                    .map(|s| Level::from_str(&s).expect("Could not parse LOG_LEVEL"))
                    .unwrap_or_else(|_| Level::Debug);
            //simple_logger::init_with_level(log_level).expect("couldn't initialize logging");
            logging::log!("Starting roadiebag with log level {:?}", log_level);*/


            if env::var("LEPTOS_ENVIRONMENT").expect("Failed to find LEPTOS_ENVIRONMENT Env Var").to_lowercase() == "local" {
                println!("LOCAL ENVIRONMENT");
                init_subscriber(get_subscriber(
                    "roadiebag".into(),
                    "INFO".into(),
                    std::io::stdout,
                ));
            } else if env::var("LEPTOS_ENVIRONMENT").expect("Failed to find LEPTOS_ENVIRONMENT Env Var") == "prod_no_trace" {
                init_subscriber(get_subscriber(
                    "roadiebag".into(),
                    "INFO".into(),
                    std::io::stdout,
                 ));
            } else{
                init_subscriber(
                    get_subscriber_with_tracing(
                        "roadiebag".into(),
                        "INFO".into(),
                        std::io::stdout,
                    ).await);
            }
            tracing::info!("Telemetry started");
        }

        pub async fn get_db_pool() -> SqlitePool {
            let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL");
            let pool = SqlitePoolOptions::new()
                .connect(&database_url)
                .await
                .expect("Could not make connection pool.");


            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("could not run SQLx migrations");

            pool
        }

        pub async fn load_leptos_options(fname: Option<&str>, site_addr: Option<String>) -> LeptosOptions {
            let mut conf = get_configuration(fname).await.unwrap();
            if let Some(addr) = site_addr {
                conf.leptos_options.site_addr = addr.parse().unwrap();
            }
            conf.leptos_options
        }

        pub fn get_app_state(pool: SqlitePool, options: LeptosOptions) -> AppState {
            let addr = options.site_addr;
            let routes = generate_route_list(App);
            AppState {
                leptos_options: options,
                pool: pool.clone(),
                routes: routes.clone(),
            }
        }

        pub async fn get_router(app_state: AppState) -> Router {
            let session_config = SessionConfig::default().with_table_name("axum_sessions");
            let auth_config = AuthConfig::<i64>::default();
            let session_store = SessionStore::<SessionSqlitePool>::new(Some(app_state.pool.clone().into()), session_config).await.unwrap();

            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(app_state.routes.clone(), get(leptos_routes_handler) )
                .layer(TraceLayer::new_for_http())
                .fallback(file_and_error_handler)
                .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(app_state.pool.clone()))
                    .with_config(auth_config))
                .layer(SessionLayer::new(session_store))
                .with_state(app_state);
            app
        }


    }
}