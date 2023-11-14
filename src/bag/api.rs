use cfg_if::cfg_if;
use leptos::*;

use std::collections::HashMap;
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BagItemForm {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) quantity: i32,
    pub(crate) size: ItemSize,
    pub(crate) infinite: bool,
}

impl BagItemForm {
    pub fn validate(&self) -> Option<RoadieAppError> {
        let mut error_map = HashMap::new();
        if self.name.trim().is_empty() {
            error_map.insert("name".to_string(), RoadieAppError::ItemNameNonEmpty.to_string());
        }
        if self.size == ItemSize::Unknown {
            error_map.insert("size".to_string(), RoadieAppError::ItemSizeMustBeSet.to_string());
        }
        if self.quantity.clone() <= 0 {
            error_map.insert("quantity".to_string(), RoadieAppError::ItemQntGtZero.to_string());
        }
        if !error_map.is_empty() {
            Some(RoadieAppError::MultipleErrors(error_map))
        } else {
            None
        }
    }
}

impl Default for BagItemForm {
    fn default() -> Self {
        BagItemForm {
            id: -1,
            quantity: 1,
            name: "".to_string(),
            description: "".to_string(),
            size: ItemSize::Unknown,
            infinite: false
        }
    }
}

impl From<BagItem> for BagItemForm {
    fn from(value: BagItem) -> Self {
        BagItemForm {
            id: value.id,
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            size: value.size,
            infinite: value.infinite
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(CreateUpdateBagItem, "/api", "Url", "create_bag_item")]
pub async fn create_update_bag_item(item: BagItemForm) -> Result<RoadieResult<BagItemForm>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();
    let mut item = item;
    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let errors = item.validate();
        if errors.is_some() {
            response.set_status(StatusCode::BAD_REQUEST);
            Ok(Err(errors.unwrap()))
        } else {
            if item.id == -1 {
                let bi = BagItem {
                    id: -1,
                    name: item.name.clone(),
                    added_by: auth.current_user.unwrap(),
                    description: item.description.clone(),
                    infinite: item.infinite,
                    quantity: item.quantity,
                    size: item.size,
                    created_at: Utc::now()
                };
                let insert_item = bi.insert(&pool).await?;
                item.id = insert_item.id;
                tracing::info!("Item with ID {} added", &item.id);
                Ok(Ok(item))
            } else {
                match BagItem::by_id(item.id, &pool).await? {
                    Some(mut e) => {
                        tracing::info!("Updating item ID {}", item.id);
                        e.name = item.name.clone();
                        e.description = item.description.clone();
                        e.infinite = item.infinite;
                        e.quantity = item.quantity;
                        e.size = item.size;
                        e.update(&pool).await?;
                        Ok(Ok(item))
                    },
                    None => {
                        tracing::error!("Unable to find item {}", item.id);
                        response.set_status(StatusCode::NOT_FOUND);
                        Ok(Err(RoadieAppError::NotFound))
                    }
                }
            }
        }
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetBagItem, "/api", "Url", "get_bag_item")]
pub async fn get_bag_item(item_id: i64) -> Result<RoadieResult<BagItem>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
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
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(DeleteBagItem, "/api", "Url", "delete_bag_item")]
pub async fn delete_bag_item(item_id: i64) -> Result<RoadieResult<()>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
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

#[tracing::instrument(level = "info", fields(error), err)]
#[server(ListBagItems, "/api", "Url", "list_bag_items")]
pub async fn list_bag_items(filter: Option<BagItemFilter>)
                            -> Result<RoadieResult<BagItemPage>, ServerFnError> {
    let pool = db_pool()?;

    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let page = BagItem::filter(filter.unwrap_or_default(), &pool).await?;
        Ok(Ok(page))
    }
}


#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(TakeRandom, "/api", "Url", "take_random")]
pub async fn take_random() -> Result<RoadieResult<Option<TakenBagItem>>, ServerFnError> {
    let pool = db_pool()?;
    let auth = auth_session()?;
    let response = expect_context::<ResponseOptions>();

    if auth.is_anonymous() {
        response.set_status(StatusCode::UNAUTHORIZED);
        leptos_axum::redirect("/auth");
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
        leptos_axum::redirect("/auth");
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
        leptos_axum::redirect("/auth");
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
        leptos_axum::redirect("/auth");
        Ok(Err(RoadieAppError::Unauthorized))
    } else {
        let items = TakenBagItem::for_item(item_id, &pool).await?;
        Ok(Ok(items))
    }
}