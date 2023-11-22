use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

#[cfg(test)]
pub(crate) mod tests {

    use axum_test::TestServer;
    use sqlx::SqlitePool;
    use crate::auth::model::{User, UserTable, SQLUser};
    use crate::errors::*;
    use sea_query::{
        SqliteQueryBuilder,
        Query,
        IdenStatic
    };
    use sqlx::prelude::*;
    use sea_query_binder::SqlxBinder;
    use leptos::logging;
    use serde::{Serialize, Deserialize};
    use http::StatusCode;
    use anyhow::*;
    use crate::tests::tests::get_test_server;
    use std::result::Result::Ok;

    #[tracing::instrument(level = "info", skip_all, err)]
    #[sqlx::test]
    async fn test_user_model(pool: SqlitePool) -> Result<()> {
        let user_id = SQLUser::create("myuser".into(), "mypassword".into(), &pool).await?;

        let (q, v) = Query::select()
            .from(UserTable::Table)
            .columns([UserTable::Username])
            .to_owned()
            .build_sqlx(SqliteQueryBuilder);

        let result = sqlx::query_with(&q, v)
            .fetch_one(&pool)
            .await?;

        assert_eq!(result.get::<String, _>(UserTable::Username.as_str()), "myuser");

        let user_by_id = SQLUser::by_id(user_id, &pool).await?;
        assert_eq!(user_by_id.is_some(), true);
        let user_by_id = user_by_id.unwrap();
        assert_eq!(user_by_id.id, user_id);
        assert_eq!(user_by_id.username, "myuser");

        let bad_user = SQLUser::by_id(user_id+1, &pool).await?;
        assert_eq!(bad_user.is_some(), false);

        let user_by_name = SQLUser::by_username("myuser".into(), &pool).await?;
        assert_eq!(user_by_name.is_some(), true);
        let user_by_name = user_by_name.unwrap();
        assert_eq!(user_by_name.id, user_id);
        assert_eq!(user_by_name.username, "myuser");

        let bad_user_name = SQLUser::by_username("foo".into(), &pool).await?;
        assert_eq!(bad_user_name.is_some(), false);
        Ok(())
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct SignupTest {
        username: String,
        password: String,
        password_confirmation: String
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct LoginTest {
        username: String,
        password: String,
        remember: Option<String>
    }

    pub(crate) async fn create_test_user(server: &TestServer, uname:Option<String>) -> User {
        let uname = uname.unwrap_or("scott".into());
        logging::log!("Creating test user {}", uname);
        let response = server.post("/api/auth_logout")
            .await;
        response.assert_status(StatusCode::FOUND);
        let response = server.post("/api/auth_signup")
            .form(&SignupTest {
                username: uname.clone(),
                password: "1234".into(),
                password_confirmation: "1234".into()
            })
            .await;
        response.assert_status(StatusCode::SEE_OTHER);
        let response2 = server.post("/api/auth_login")
            .form(&LoginTest {
                username: uname.clone(),
                password: "1234".into(),
                remember: Some("yes".into())
            })
            .await;
        response2.assert_status(StatusCode::SEE_OTHER);
        let user_response = server.get("/api/get_user").await;
        let u = user_response.json::<User>();
        assert_eq!(u.username, uname);
        u
    }


    #[tracing::instrument(level = "info", skip_all, err)]
    #[sqlx::test]
    async fn test_user_e2e(pool: SqlitePool) -> Result<()> {
        let test_server = get_test_server(&pool).await?;

        let user_response = test_server.get("/api/get_user").await;
        let user = user_response.json::<User>();
        assert_eq!(user.anonymous, true);


        let response = test_server.post("/api/auth_signup")
            .form(&SignupTest {
                username: "scott".into(),
                password: "1234".into(),
                password_confirmation: "1234".into()
            })
            .await;
        response.assert_status(StatusCode::SEE_OTHER);


        let response2 = test_server.post("/api/auth_login")
            .form(&LoginTest {
                username: "scott".into(),
                password: "1234".into(),
                remember: Some("yes".into())
            })
            .await;
        response2.assert_status(StatusCode::SEE_OTHER);

        let user_response = test_server.get("/api/get_user").await;
        let user = user_response.json::<User>();
        assert_eq!(user.anonymous, false);
        assert_eq!(user.username, "scott");

        let response3 = test_server.post("/api/auth_logout").await;
        response3.assert_status(StatusCode::FOUND);

        let user_response = test_server.get("/api/get_user").await;
        let user = user_response.json::<User>();
        assert_eq!(user.anonymous, true);

        let bpresponse = test_server.post("/api/auth_login")
            .form(&LoginTest {
                username: "scott".into(),
                password: "123".into(),
                remember: Some("yes".into())
            })
            .await;

        bpresponse.assert_status(StatusCode::UNAUTHORIZED);

        Ok(())
    }

    #[tracing::instrument(level = "info", skip_all, err)]
    #[sqlx::test]
    async fn test_bad_user(pool: SqlitePool) -> Result<()> {
        let test_server = get_test_server(&pool).await?;

        let response = test_server.post("/api/auth_signup")
            .form(&SignupTest {
                username: "scott".into(),
                password: "".into(),
                password_confirmation: "1234".into()
            })
            .await;
        response.assert_status(StatusCode::BAD_REQUEST);
        let resp_obj = response.json::<RoadieResult<()>>();
        assert_eq!(resp_obj, Err(RoadieAppError::ValidationFailedForField("password".into())));
        Ok(())
    }
}
}}
