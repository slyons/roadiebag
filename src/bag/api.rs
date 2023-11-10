use cfg_if::cfg_if;
use leptos::*;


use serde::{Serialize, Deserialize};
use super::model::*;
use crate::errors::*;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::auth::auth_session;
        use crate::db::db_pool;
        use chrono::Utc;
        use http::status::StatusCode;
        use leptos_axum::ResponseOptions;
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BagItemForm {
    pub(crate) id: Option<i64>,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) quantity: i32,
    pub(crate) size: ItemSize,
    pub(crate) infinite: bool,
}

impl From<BagItem> for BagItemForm {
    fn from(value: BagItem) -> Self {
        BagItemForm {
            id: Some(value.id),
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            size: value.size,
            infinite: value.infinite
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(CreateBagItem, "/api", "Url", "create_bag_item")]
pub async fn create_bag_item(item: BagItemForm) -> Result<RoadieResult<BagItem>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let bi = BagItem {
            id: -1,
            name: item.name,
            added_by: auth.current_user.unwrap(),
            description: item.description,
            infinite: item.infinite,
            quantity: item.quantity,
            size: item.size,
            created_at: Utc::now()
        };
        logging::log!("Item is {:?}", &bi);
        let item = bi.insert(&pool).await?;
        Ok(Ok(item))
    }
}
#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(UpdateBagItem, "/api", "Url", "update_bag_item")]
pub async fn update_bag_item(item: BagItemForm) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        if item.id == Some(-1) || item.id.is_none() {
            Ok(Err(RoadieAppError::ValidationFailedError))
        } else {
            match BagItem::by_id(item.id.unwrap(), &pool).await? {
                Some(mut e) => {
                    e.name = item.name;
                    e.description = item.description;
                    e.infinite = item.infinite;
                    e.quantity = item.quantity;
                    e.size = item.size;
                    e.update(&pool).await?;
                    Ok(Ok(()))
                },
                None => {
                    response.set_status(StatusCode::NOT_FOUND);
                    Ok(Err(RoadieAppError::NotFound))
                }
            }
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetBagItem, "/api", "Url", "get_bag_item")]
pub async fn get_bag_item(item_id: i64) -> Result<RoadieResult<BagItem>, ServerFnError> {
    let pool = db_pool()?;
    let response = expect_context::<ResponseOptions>();

    let item = BagItem::by_id(item_id, &pool).await?;
    match item {
        Some(bi) => {
            response.set_status(StatusCode::OK);
            Ok(Ok(bi))
        },
        None => {
            response.set_status(StatusCode::NOT_FOUND);
            Ok(Err(RoadieAppError::NotFound))
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(DeleteBagItem, "/api", "Url", "delete_bag_item")]
pub async fn delete_bag_item(item_id: i64) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let item = BagItem::by_id(item_id, &pool).await?;
        match item {
            Some(bi) => {
                bi.delete(&pool).await?;
                response.set_status(StatusCode::OK);
                Ok(Ok(()))
            },
            None => {
                response.set_status(StatusCode::NOT_FOUND);
                Ok(Err(RoadieAppError::NotFound))
            }
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ListBagItems, "/api", "Url", "list_bag_items")]
pub async fn list_bag_items(filter: Option<BagItemFilter>)
                            -> Result<RoadieResult<BagItemPage>, ServerFnError> {
    let pool = db_pool()?;

    let page = BagItem::filter(filter.unwrap_or_default(), &pool).await?;
    Ok(Ok(page))
}


#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(TakeRandom, "/api", "Url", "take_random")]
pub async fn take_random() -> Result<RoadieResult<Option<TakenBagItem>>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        Ok(Ok(TakenBagItem::get_random(&pool).await?))
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(UpdateTaken, "/api", "Url", "update_taken")]
pub async fn update_taken(taken_item: TakenBagItem) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        Ok(Ok(taken_item.update(&pool).await?))
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(LastTaken, "/api", "Url", "last_taken")]
pub async fn last_taken() -> Result<RoadieResult<Option<TakenBagItem>>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let item = TakenBagItem::last(&pool).await?;
        Ok(Ok(item))
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(ForItem, "/api", "Url", "for_item")]
pub async fn for_item(item_id: i64) -> Result<RoadieResult<Vec<TakenBagItem>>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let items = TakenBagItem::for_item(item_id, &pool).await?;
        Ok(Ok(items))
    }
}