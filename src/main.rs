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
        use roadiebag::app::App;
        use roadiebag::auth::*;
        use roadiebag::auth::User;
        use roadiebag::bag::*;
        use roadiebag::state::AppState;
        use roadiebag::fallback::file_and_error_handler;

        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use leptos::{logging::log, provide_context, get_configuration};
        use log::Level;
        use std::str::FromStr;
        use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthSessionLayer, AuthConfig, SessionSqlitePool};
        use dotenvy::dotenv;
        use std::env;

        async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession,
            path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>)
        -> impl IntoResponse {

            log!("{:?}", path);

            handle_server_fns_with_context(path, headers, raw_query, move || {
                provide_context(auth_session.clone());
                provide_context(app_state.pool.clone());
            }, request).await
        }

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

        #[tokio::main]
        async fn main() {
            dotenv().ok();
            let log_level = env::var("LOG_LEVEL")
                    .map(|s| Level::from_str(&s).expect("Could not parse LOG_LEVEL"))
                    .unwrap_or_else(|_| Level::Debug);
            simple_logger::init_with_level(log_level).expect("couldn't initialize logging");
            logging::log!("Starting roadiebag with log level {:?}", log_level);

            let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL");
            let pool = SqlitePoolOptions::new()
                .connect(&database_url)
                .await
                .expect("Could not make connection pool.");

            let session_config = SessionConfig::default().with_table_name("axum_sessions");
            let auth_config = AuthConfig::<i64>::default();
            let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config).await.unwrap();

            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("could not run SQLx migrations");

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(App);

            let app_state = AppState{
                leptos_options,
                pool: pool.clone(),
                routes: routes.clone(),
            };

            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
                .fallback(file_and_error_handler)
                .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
                .with_config(auth_config))
                .layer(SessionLayer::new(session_store))
                .with_state(app_state);

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