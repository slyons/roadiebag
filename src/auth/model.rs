use cfg_if::cfg_if;

use serde::{Deserialize, Serialize};
cfg_if! {
    if #[cfg(feature="ssr")] {
        use sea_query_binder::SqlxBinder;
        #[cfg(feature="derive")]
        use sea_query::*;
        use sea_query::{Query, Expr, IdenStatic, Func, SqliteQueryBuilder, SelectStatement, Asterisk};

        use bcrypt::{hash, DEFAULT_COST};
        use sqlx::SqlitePool;
        use sqlx::Row;
        use sqlx::prelude::*;
        use axum_session_auth::{Authentication, HasPermission};
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub anonymous: bool
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            username: "Guest".into(),
            anonymous: true
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use async_trait::async_trait;

        /*use crate::db::FromRowPrefix;
        impl<'r, R> FromRowPrefix<'r, R> for User
        where R: Row{
            fn from_row_prefix(row: &'r R, prefix: String) -> Result<Self, sqlx::Error> {
                Ok(User {
                    id: row.try_get(format!("`{}`.`{}`", prefix, "id"))?,
                    username: row.try_get(format!("`{}`.`{}`", prefix, "username"))?,
                    anonymous: false
                })
            }
        }*/


        #[derive(IdenStatic, Copy, Clone)]
        #[iden="users"]
        pub enum UserTable {
            Table,
            Id,
            Username,
            Password
        }

        #[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct SQLUser {
            pub id: i64,
            pub username: String,
            pub password: String,
        }

        impl Into<User> for SQLUser {
            fn into(self) -> User {
                User {
                    id: self.id,
                    username: self.username,
                    anonymous: false
                }
            }
        }

        impl SQLUser {
            pub async fn create<S: Into<String>>(username: S, password: S, pool: &SqlitePool) -> Result<i64, sqlx::Error> {
                let password_hashed = hash(password.into(), DEFAULT_COST).unwrap();

                let (insert_stmt, values) = Query::insert()
                    .into_table(UserTable::Table)
                    .columns([UserTable::Username, UserTable::Password])
                    .values_panic([username.into().to_lowercase().into(), password_hashed.into()])
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let id = sqlx::query_with(&insert_stmt, values)
                    .execute(pool)
                    .await?
                    .last_insert_rowid();
                Ok(id)
            }

            async fn get_one(mut query: SelectStatement, pool: &SqlitePool) -> Result<Option<Self>, sqlx::Error> {
                logging::debug_warn!("get one");
                let mut user_vec = Self::get_many(query.limit(1).take(), pool).await?;
                if user_vec.len() >= 1 {
                    Ok(Some(user_vec.remove(0)))
                } else {
                    Ok(None)
                }
            }

            async fn get_many(query: SelectStatement, pool: &SqlitePool) -> Result<Vec<SQLUser>, sqlx::Error> {
                let (sql, values):(String, _) = query.build_sqlx(SqliteQueryBuilder);
                logging::debug_warn!("User query is {:?}", &sql);
                let result = sqlx::query_as_with::<_, SQLUser, _>(&sql, values)
                    .fetch_all(pool)
                    .await;
                result.map_err(|e| {
                    logging::error!("Error when executing query: {:?}",e);
                    e.into()
                })
            }

            pub async fn by_id(id: i64, pool: &SqlitePool) -> Result<Option<SQLUser>, sqlx::Error> {
                logging::debug_warn!("by id");
                Self::get_one(
                    Query::select()
                        .column(Asterisk)
                        .from(UserTable::Table)
                        .and_where(Expr::col(UserTable::Id).eq(id))
                        .to_owned(),
                    pool
                    )
                    .await
            }

            pub async fn by_username<S: Into<String>>(uname: S, pool: &SqlitePool) -> Result<Option<SQLUser>, sqlx::Error> {
                logging::debug_warn!("by username");
                Self::get_one(
                    Query::select()
                        .column(Asterisk)
                        .from(UserTable::Table)
                        .and_where(
                            Expr::expr(
                                Func::lower(Expr::col(UserTable::Username))
                            )
                            .eq(uname.into().trim().to_lowercase())
                        )
                        .to_owned(),
                    pool
                    )
                    .await
            }

        }



        #[derive(IdenStatic, Copy, Clone)]
        #[iden="user_permissions"]
        pub enum UserPermissionsTable {
            Table,
            UserId,
            Token
        }

        /*#[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct SQLUserPermission {
            pub user_id: i64,
            pub token: String,
        }

        impl SQLUserPermission {

        }*/

        #[async_trait]
        impl Authentication<User, i64, SqlitePool> for User {
            async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
                let pool = pool.unwrap();

                let user = SQLUser::by_id(userid, pool)
                    .await?;
                match user {
                    Some(u) => Ok(User {
                    username: u.username,
                    id: u.id,
                    anonymous: false
                }),
                    None => Ok(User::default())
                }
            }

            fn is_authenticated(&self) -> bool {
                !self.anonymous
            }

            fn is_active(&self) -> bool {
                true
            }

            fn is_anonymous(&self) -> bool {
                self.anonymous
            }
        }

        #[async_trait]
        impl HasPermission<SqlitePool> for User {
            async fn has(&self, _perm: &str, _pool: &Option<&SqlitePool>) -> bool {
                true
            }
        }
    }
}