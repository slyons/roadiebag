use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use strum::EnumIter;

use crate::auth::User;
use crate::errors::{RoadieAppError, RoadieResult};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, EnumIter)]
enum ItemSize {
    Small,
    Medium,
    Large,
    Unkown
}

impl From<u8> for ItemSize {
    fn from(value: u8) -> Self {
        match value {
            0 => ItemSize::Small,
            1 => ItemSize::Medium,
            2 => ItemSize::Large,
            _ => ItemSize::Unkown
        }
    }
}

impl Into<u8> for ItemSize {
    fn into(self) -> u8 {
        match self {
            ItemSize::Small => 0,
            ItemSize::Medium => 1,
            ItemSize::Large => 2,
            ItemSize::Unkown => 99
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BagItem {
    id: i64,
    added_by: User,
    name: String,
    description: String,
    quantity: i32,
    size: u8,
    infinite: bool,
    created_at: DateTime<Utc>
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TakenBagItem {
    item: BagItem,
    rounds: u32
}

#[derive(Serialize, Default, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BagItemFilter {
    added_by: Option<Vec<i64>>,
    name: Option<String>,
    description: Option<String>,
    size: Option<Vec<u8>>,
    infinite: Option<bool>
}

impl BagItemFilter {
    fn any_filter(&self) -> bool {
        self.added_by.is_some() ||
            self.name.is_some() ||
            self.description.is_some() ||
            self.size.is_some() ||
            self.infinite.is_some()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BagItemPage {
    items: Vec<BagItem>,
    page_num: u64,
    total_pages: u64,
    pagesize: u64,
    total_results: u64
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use leptos_axum::ResponseOptions;
        use sqlx::prelude::*;
        use sqlx::SqlitePool;
        use crate::db::db_pool;
        use sea_query_binder::SqlxBinder;
        #[cfg(feature="derive")]
        use sea_query::*;
        use sea_query::{Query, Expr, Iden, IdenStatic,
            Func, SqliteQueryBuilder, SelectStatement, Order};
        use sea_query::types::{Alias, Asterisk};
        use crate::auth::model::UserTable;
        use crate::auth::auth_session;
        use rand::Rng;

        #[derive(IdenStatic, EnumIter, Copy, Clone)]
        #[iden="bagitems"]
        pub enum BagItemsTable {
            Table,
            Id,
            #[iden="added_by"]
            AddedBy,
            Name,
            Description,
            Quantity,
            Size,
            Infinite,
            #[iden="created_at"]
            CreatedAt
        }

        impl BagItem {
            async fn insert(self, pool:&SqlitePool) -> Result<BagItem, sqlx::Error> {
                let (insert_stmt, values) = Query::insert()
                    .into_table(BagItemsTable::Table)
                    .columns([
                        BagItemsTable::AddedBy,
                        BagItemsTable::Name,
                        BagItemsTable::Description,
                        BagItemsTable::Quantity,
                        BagItemsTable::Size,
                        BagItemsTable::Infinite,
                        BagItemsTable::CreatedAt
                    ])
                    .values_panic([
                        self.added_by.id.into(),
                        (&self.name).into(),
                        (&self.description).into(),
                        self.quantity.into(),
                        Into::<u8>::into(self.size).into(),
                        self.infinite.into(),
                        self.created_at.into()
                    ])
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);

                let result = sqlx::query_with(&insert_stmt, values)
                    .fetch_one(pool)
                    .await?;
                let item_id:i64 = result.get(0);
                Ok(BagItem{
                    id: item_id,
                    ..self
                })
            }

            async fn update(&self, pool:&SqlitePool) -> Result<(), sqlx::Error> {
                let (q, values) = Query::update()
                    .table(BagItemsTable::Table)
                    .values([
                        (BagItemsTable::AddedBy, self.added_by.id.into()),
                        (BagItemsTable::Name, (&self.name).into()),
                        (BagItemsTable::Description,(&self.description).into()),
                        (BagItemsTable::Quantity, self.quantity.into()),
                        (BagItemsTable::Size, Into::<u8>::into(self.size).into()),
                        (BagItemsTable::Infinite, self.infinite.into()),
                        (BagItemsTable::CreatedAt, self.created_at.into())
                    ])
                    .and_where(Expr::col(BagItemsTable::Id).eq(self.id))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);

                sqlx::query_with(&q, values)
                    .fetch_one(pool)
                    .await?;
                Ok(())
            }

            async fn delete(self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
                let (q, values) = Query::delete()
                    .from_table(BagItemsTable::Table)
                    .cond_where(Expr::col(BagItemsTable::Id).eq(self.id))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                sqlx::query_with(&q, values)
                    .fetch_one(pool)
                    .await?;
                Ok(())
            }

            async fn get_one(mut query: SelectStatement, pool: &SqlitePool) -> Result<Option<Self>, sqlx::Error> {
                let mut item_vec = Self::get_many(query.limit(1).take(), pool).await?;
                if item_vec.len() >= 1 {
                    Ok(Some(item_vec.remove(0)))
                } else {
                    Ok(None)
                }
            }

            async fn get_many(mut query: SelectStatement, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
                let (q, values) = query
                    .from(BagItemsTable::Table)
                    .column(Asterisk)
                    .expr_as(Expr::col((UserTable::Table, UserTable::Id)), Alias::new("user_id"))
                    .column((UserTable::Table, UserTable::Username))
                    .inner_join(
                        UserTable::Table,
                        Expr::col((BagItemsTable::Table, BagItemsTable::CreatedAt)).equals((UserTable::Table, UserTable::Id))
                    ).to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let result = sqlx::query_with(&q, values)
                    .fetch_all(pool)
                    .await?;
                    Ok(result.iter().map(|row| {
                        BagItem {
                            id: row.get(BagItemsTable::Id.as_str()),

                            added_by: User {
                                id: row.get("user_id"),
                                username: row.get("username"),
                                anonymous: false
                            },
                            name: row.get(BagItemsTable::Name.as_str()),
                            description: row.get(BagItemsTable::Description.as_str()),
                            quantity: row.get(BagItemsTable::Quantity.as_str()),
                            size: row.get::<u8, _>(BagItemsTable::Size.as_str()).into(),
                            infinite: row.get(BagItemsTable::Infinite.as_str()),
                            created_at: row.get::<DateTime<Utc>, _>(BagItemsTable::CreatedAt.as_str())
                        }
                    }).collect())
            }

            async fn by_id(id: i64, pool: &SqlitePool) -> Result<Option<BagItem>, sqlx::Error> {
                Self::get_one(
                    Query::select()
                        .and_where(Expr::col(BagItemsTable::Id).eq(id)).to_owned(),
                    pool
                ).await
            }

            async fn count(query: Option<SelectStatement>, pool: &SqlitePool) -> Result<u64, sqlx::Error> {
                let mut query = query.unwrap_or(Query::select());
                let (q, v) = query
                    .from(BagItemsTable::Table)
                    .expr_as(Expr::col(BagItemsTable::Id).count(), Alias::new("itemcount"))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);

                sqlx::query_with(&q, v)
                    .fetch_one(pool)
                    .await
                    .map(|r| {
                        r.get::<i64, _>("itemcount")
                            .try_into()
                            .expect("Underflow")
                    })

            }

            async fn filter(filter: BagItemFilter, pagesize: u64, page: u64, pool: &SqlitePool) -> Result<BagItemPage, sqlx::Error>{
                let mut query = Query::select();
                if filter.any_filter() {
                    if let Some(added_by) = filter.added_by {
                        query = query.and_where(Expr::col(BagItemsTable::AddedBy).is_in(added_by)).take();
                    }
                    if let Some(name) = filter.name {
                        query = query.and_where(Expr::col(BagItemsTable::Name).like(name)).take();
                    }
                    if let Some(description) = filter.description {
                        query = query.and_where(Expr::col(BagItemsTable::Description).like(description)).take();
                    }
                    if let Some(size) = filter.size {
                        query = query.and_where(Expr::col(BagItemsTable::Size).is_in(size)).take();
                    }
                    if let Some(infinite) = filter.infinite {
                        query = query.and_where(Expr::col(BagItemsTable::Infinite).eq(infinite)).take();
                    }
                }
                let count = Self::count(Some(query.clone()), pool).await?;
                let page = if page <= 0 {0} else {page - 1};
                let offset:u64 = page * pagesize.clone();
                query = query
                    .offset(offset)
                    .limit(pagesize)
                    .to_owned();
                let items = Self::get_many(query, pool).await?;
                Ok(BagItemPage {
                    items: items,
                    page_num: page,
                    pagesize: pagesize,
                    total_pages: count.div_ceil(pagesize),
                    total_results: count
                })
            }

            async fn take_random(pool: &SqlitePool) -> Result<TakenBagItem, sqlx::Error> {
                let mut base_query = Query::select()
                    .from(BagItemsTable::Table)
                    .order_by(BagItemsTable::Id, Order::Desc)
                    .and_where(Expr::col(BagItemsTable::Quantity).gte(1).or(Expr::col(BagItemsTable::Infinite).eq(true)))
                    .to_owned();
                let (count_query, v) = base_query.clone()
                    .expr_as(Expr::col(BagItemsTable::Id).count(), Alias::new("itemcount"))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let count:u64 = sqlx::query_with(&count_query, v)
                    .fetch_one(pool)
                    .await?
                    .get::<i64, _>("itemcount")
                    .try_into()
                    .expect("Underflow");
                let mut rng = rand::thread_rng();
                let item_offset = rng.gen_range(0..count);
                logging::log!("{} items are eligible, picking number {}", count, item_offset);

                let (q, v) = base_query
                    .column(BagItemsTable::Id)
                    .offset(item_offset)
                    .limit(1)
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let result = sqlx::query_with(&q, v)
                    .fetch_one(pool)
                    .await?;
                let item_id:i64 = result.get("id");
                logging::log!("Selecting item ID {}", item_id);
                let mut item = Self::by_id(item_id, pool).await?.expect(&format!("Invalid item ID {}", item_id));
                item.quantity -= 1;
                item.update(pool).await?;

                Ok(TakenBagItem {
                    item: item,
                    rounds: rng.gen_range(1..=6)
                })
            }
        }
    }
}

#[server(CreateBagItem, "/api")]
pub async fn create_bag_item(item: BagItem) -> Result<RoadieResult<BagItem>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;

    if auth.is_anonymous() {
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        if item.id != -1 {
            Ok(Err(RoadieAppError::ValidationFailedError))
        } else {
            let item = item.insert(&pool).await?;
            Ok(Ok(item))
        }
    }
}

#[server(EditBagItem, "/api")]
pub async fn edit_bag_item(item: BagItem) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;

    if auth.is_anonymous() {
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        if item.id == -1 {
            Ok(Err(RoadieAppError::ValidationFailedError))
        } else {
            item.update(&pool).await?;
            Ok(Ok(()))
        }
    }
}

#[server(DeleteBagItem, "/api")]
pub async fn delete_bag_item(item: BagItem) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;

    if auth.is_anonymous() {
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        item.delete(&pool).await?;
        Ok(Ok(()))
    }
}

#[server(ListBagItems, "/api")]
pub async fn list_bag_items(page_num: u64, limit: u64, filter: Option<BagItemFilter>)
    -> Result<RoadieResult<BagItemPage>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;

    if auth.is_anonymous() {
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let page = BagItem::filter(filter.unwrap_or_default(), limit, page_num, &pool).await?;
        Ok(Ok(page))
    }
}