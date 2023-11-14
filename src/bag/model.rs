use serde::{Deserialize, Serialize};
use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use strum::*;



use crate::auth::User;
use strum::Display;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Copy, EnumIter, Display, EnumString, FromRepr)]
pub enum ItemSize {
    Small,
    Medium,
    Large,
    //#[strum(disabled)]
    Unknown
}

impl Default for ItemSize {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u8> for ItemSize {
    fn from(value: u8) -> Self {
        match value {
            0 => ItemSize::Small,
            1 => ItemSize::Medium,
            2 => ItemSize::Large,
            _ => ItemSize::Unknown
        }
    }
}

impl Into<u8> for ItemSize {
    fn into(self) -> u8 {
        match self {
            ItemSize::Small => 0,
            ItemSize::Medium => 1,
            ItemSize::Large => 2,
            ItemSize::Unknown => 99
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BagItem {
    pub(crate) id: i64,
    pub(crate) added_by: User,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) quantity: i32,
    pub(crate) size: ItemSize,
    pub(crate) infinite: bool,
    pub(crate) created_at: DateTime<Utc>
}

#[derive(Serialize, Default, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BagItemFilter {
    pub added_by: Option<Vec<i64>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub size: Option<Vec<u8>>,
    pub infinite: Option<bool>,
    pub page_size: Option<u64>,
    pub page_num: Option<u64>
}

impl BagItemFilter {
    pub fn with_page(&self, page_num: u64) -> Self {
        BagItemFilter {

            page_num: Some(page_num),
            added_by: self.added_by.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            size: self.size.clone(),
            infinite: self.infinite.clone(),
            page_size: self.page_size.clone()
        }
    }
}

#[derive(Serialize, Default, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BagItemPage {
    pub items: Vec<BagItem>,
    pub page_num: u64,
    pub total_pages: u64,
    pub page_size: u64,
    pub total_results: u64
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use sqlx::prelude::*;
        use sqlx::SqlitePool;
        use futures::future::try_join_all;

        use sea_query_binder::SqlxBinder;
        #[cfg(feature="derive")]
        use sea_query::*;
        use sea_query::{Query, Expr, IdenStatic,
            Func, SqliteQueryBuilder, SelectStatement, Order, JoinType};
        use sea_query::types::{Alias, Asterisk};
        use crate::auth::model::UserTable;
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
            #[tracing::instrument(level = "info", skip_all, ret, err)]
            pub async fn insert(self, pool:&SqlitePool) -> Result<BagItem, sqlx::Error> {
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

                let row_id = sqlx::query_with(&insert_stmt, values)
                    .execute(pool)
                    .await?
                    .last_insert_rowid();

                Ok(BagItem{
                    id: row_id,
                    ..self
                })
            }

            #[tracing::instrument(level = "info", skip_all, ret, err)]
            pub async fn update(&self, pool:&SqlitePool) -> Result<(), sqlx::Error> {
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
                    .execute(pool)
                    .await?;
                Ok(())
            }

            #[tracing::instrument(level = "info", skip_all, fields(error), ret, err)]
            pub async fn delete(self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
                let (q, values) = Query::delete()
                    .from_table(BagItemsTable::Table)
                    .cond_where(Expr::col(BagItemsTable::Id).eq(self.id))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                sqlx::query_with(&q, values)
                    .execute(pool)
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
                    .column((BagItemsTable::Table, Asterisk))
                    .expr_as(Expr::col((UserTable::Table, UserTable::Id)), Alias::new("user_id"))
                    .column((UserTable::Table, UserTable::Username))
                    .inner_join(
                        UserTable::Table,
                        Expr::col((BagItemsTable::Table, BagItemsTable::AddedBy)).equals((UserTable::Table, UserTable::Id))
                    )

                    .order_by((BagItemsTable::Table, BagItemsTable::Id), Order::Desc)
                    .to_owned()
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

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn by_id(id: i64, pool: &SqlitePool) -> Result<Option<BagItem>, sqlx::Error> {
                Self::get_one(
                    Query::select()
                        .and_where(Expr::col((BagItemsTable::Table, BagItemsTable::Id)).eq(id)).to_owned(),
                    pool
                ).await
            }


            pub async fn count(query: Option<SelectStatement>, pool: &SqlitePool) -> Result<u64, sqlx::Error> {
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

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn filter(filter: BagItemFilter, pool: &SqlitePool) -> Result<BagItemPage, sqlx::Error>{
                let mut query = Query::select();
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
                let count = Self::count(Some(query.clone()), pool).await?;
                let page = filter.page_num.map(|page| page - 1).unwrap_or(0);
                let page_size = filter.page_size.unwrap_or(50);
                let offset:u64 = page * page_size;
                query = query
                    .offset(offset)
                    .limit(page_size)
                    .to_owned();
                let items = Self::get_many(query, pool).await?;
                Ok(BagItemPage {
                    items: items,
                    page_num: page + 1,
                    page_size: page_size,
                    total_pages: count.div_ceil(page_size),
                    total_results: count
                })
            }

        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TakenBagItem {
    pub id: i64,
    pub item: BagItem,
    pub extraction_time: DateTime<Utc>,
    pub rounds: u32,
    pub done: bool
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        #[derive(IdenStatic, EnumIter, Copy, Clone)]
        #[iden="taken_items"]
        pub enum TakenItemsTable {
            Table,
            Id,
            #[iden="item_id"]
            ItemId,
            #[iden="extraction_time"]
            ExtractionTime,
            #[iden="num_rounds"]
            NumRounds,
            Done
        }

        impl TakenBagItem {

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn get_random(pool: &SqlitePool) -> Result<Option<TakenBagItem>, sqlx::Error> {
                let item_use_subquery = Query::select()
                    .from(TakenItemsTable::Table)
                    .column((TakenItemsTable::Table, TakenItemsTable::ItemId))
                    .expr_as(Func::count(Expr::col((TakenItemsTable::Table, TakenItemsTable::Id))), Alias::new("use_count"))
                    .group_by_col((TakenItemsTable::Table, TakenItemsTable::ItemId))
                    .to_owned();
                let mut item_uses_left = Query::select()
                    .from(BagItemsTable::Table)
                    .join_subquery(
                        JoinType::LeftJoin,
                        item_use_subquery,
                        Alias::new("usage"),
                        Expr::col(
                                (BagItemsTable::Table, BagItemsTable::Id))
                            .eq(
                                Expr::col(TakenItemsTable::ItemId)
                            )
                    )
                    .expr_as(
                        Expr::case(
                            Expr::col((BagItemsTable::Table, BagItemsTable::Infinite)).eq(false),
                            Expr::col((BagItemsTable::Table, BagItemsTable::Quantity)).sub(
                                Expr::col((Alias::new("usage"), Alias::new("use_count"))).if_null(0)
                            )
                        ).finally(1),
                        Alias::new("uses_left")
                    )
                    .columns([BagItemsTable::Id])
                    .and_where(Expr::col(Alias::new("uses_left")).gte(1))
                    .to_owned();
                let (count_query, v) = Query::select()
                    .expr_as(Expr::count(Expr::col(BagItemsTable::Id)), Alias::new("itemcount"))
                    .from_subquery(
                        item_uses_left.clone(),
                        Alias::new("uses")
                    )
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);

                let count:u64 = sqlx::query_with(&count_query, v.clone())
                    .fetch_one(pool)
                    .await?
                    .get::<i64, _>("itemcount")
                    .try_into()
                    .expect("Underflow");
                if count == 0 {
                    return Ok(None);
                }
                let mut rng = rand::thread_rng();
                let item_offset = rng.gen_range(0..count);

                let (q, v) = item_uses_left
                    .column(BagItemsTable::Id)
                    .offset(item_offset)
                    .limit(1)
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let result = sqlx::query_with(&q, v)
                    .fetch_one(pool)
                    .await?;
                let item_id:i64 = result.get("id");
                let mut item = BagItem::by_id(item_id, pool).await?.expect(&format!("Invalid item ID {}", item_id));
                if !item.infinite {
                    item.quantity -= 1;
                    item.update(pool).await?;
                }

                let num_rounds = rng.gen_range(1..=6);

                Ok(Some(Self::insert(item_id, num_rounds, pool).await?))
            }

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn insert(item_id: i64, num_rounds: i64, pool:&SqlitePool) -> Result<TakenBagItem, sqlx::Error> {
                let (q, v) = Query::insert()
                    .into_table(TakenItemsTable::Table)
                    .columns([TakenItemsTable::ItemId, TakenItemsTable::NumRounds])
                    .values_panic([
                        item_id.into(),
                        num_rounds.into()
                    ])
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let id = sqlx::query_with(&q, v)
                    .execute(pool)
                    .await?
                    .last_insert_rowid();
                Ok(Self::by_id(id, pool)
                    .await?
                    .unwrap())
            }

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn update(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
                let (q, v) = Query::update()
                    .table(TakenItemsTable::Table)
                    .values([
                        (TakenItemsTable::ItemId, self.item.id.into()),
                        (TakenItemsTable::ExtractionTime, self.extraction_time.into()),
                        (TakenItemsTable::NumRounds, self.rounds.into()),
                        (TakenItemsTable::Done, self.done.into())
                    ])
                    .and_where(Expr::col(TakenItemsTable::Id).eq(self.id))
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                sqlx::query_with(&q, v)
                    .execute(pool)
                    .await?;
                Ok(())
            }

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn by_id(id: i64, pool: &SqlitePool) -> Result<Option<Self>, sqlx::Error> {
                Self::get_one(
                    Query::select()
                        .and_where(Expr::col(TakenItemsTable::Id).eq(id))
                        .to_owned(),
                    pool
                ).await
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
                    .from(TakenItemsTable::Table)
                    .column(Asterisk)
                    .to_owned()
                    .build_sqlx(SqliteQueryBuilder);
                let result = sqlx::query_with(&q, values)
                    .fetch_all(pool)
                    .await?;
                let tbis = try_join_all(result.iter().map(|row| async {
                    let bi = BagItem::by_id(row.try_get(TakenItemsTable::ItemId.as_str())?, pool).await?;
                    Ok(TakenBagItem {
                        id: row.try_get(TakenItemsTable::Id.as_str())?,
                        item: bi.unwrap(),
                        extraction_time: row.try_get::<DateTime<Utc>, _>(TakenItemsTable::ExtractionTime.as_str())?,
                        rounds: row.try_get(TakenItemsTable::NumRounds.as_str())?,
                        done: row.try_get(TakenItemsTable::Done.as_str())?
                    })
                    }));

                tbis.await
            }

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn for_item(item_id: i64, pool: &SqlitePool) -> Result<Vec<TakenBagItem>, sqlx::Error> {
                Self::get_many(
                    Query::select()
                        .and_where(Expr::col((TakenItemsTable::Table, TakenItemsTable::ItemId)).eq(item_id))
                        .order_by((TakenItemsTable::Table, TakenItemsTable::ItemId), Order::Desc)
                        .take(),
                    pool
                ).await
            }

            #[tracing::instrument(level = "info", skip(pool), fields(error), ret, err)]
            pub async fn last(pool: &SqlitePool) -> Result<Option<Self>, sqlx::Error> {
                let tbi = Self::get_one(
                    Query::select()
                        .order_by(TakenItemsTable::Id, Order::Desc)
                        .to_owned(),
                    pool
                ).await?;

                match tbi {
                    Some(t) if t.done => Ok(None),
                    Some(t) => Ok(Some(t)),
                    None => Ok(None)
                }
            }
        }
    }
}
