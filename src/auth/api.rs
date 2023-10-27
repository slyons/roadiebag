use cfg_if::cfg_if;
use leptos::*;

use leptos_router::*;


use crate::error_template::*;
use crate::errors::*;
use crate::auth::model::User;


cfg_if! {
    if #[cfg(feature="ssr")] {
        use axum_session_auth::SessionSqlitePool;
        use http::status::StatusCode;
        use leptos_axum::*;
        use sqlx::SqlitePool;
        use crate::db::db_pool;
        use crate::auth::model::SQLUser;
        use bcrypt::{verify};

        pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
        pub fn auth_session() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing".into()))
        }
    }
}

#[server(GetUserAPI, "/api")]
pub async fn get_user() -> Result<User, ServerFnError> {
    let auth = auth_session()?;

    Ok(auth.current_user.unwrap_or_default())
}

#[server(SignupAPI, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String
) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if password != password_confirmation {
        response.set_status(StatusCode::BAD_REQUEST);
        return Ok(Err(RoadieAppError::PasswordsDoNotMatch));
    }
    
    let existing_user = SQLUser::by_username(&username, &pool).await?;
    if existing_user.is_some() {
        response.set_status(StatusCode::BAD_REQUEST);
        return Ok(Err(RoadieAppError::BadUserPassword));
    }

    SQLUser::create(username, password, &pool).await?;
    Ok(Ok(()))
}

#[server(LoginAPI, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>
) -> Result<RoadieResult<User>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;

    if !auth.is_anonymous() {
        return Ok(Ok(auth.current_user.unwrap()))
    }

    let user = SQLUser::by_username(&username, &pool)
        .await?;

    match user {
        Some(u) => {
            match verify(password, &u.password) {
                Ok(true) => {
                    auth.login_user(u.id.clone());
                    auth.remember_user(remember.is_some());
                    Ok(Ok(u.into()))
                }
                Ok(false) => {
                    Ok(Err(RoadieAppError::BadUserPassword))
                }
                Err(e) => {
                    logging::error!("BCrypt error: {:?}", e);
                    Err(ServerFnError::ServerError("BCrypt error".to_string()))
                }
            }
        },
        None => Ok(Err(RoadieAppError::BadUserPassword))
    }
}

#[server(LogoutAPI, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth_session()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}