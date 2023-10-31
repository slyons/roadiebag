use cfg_if::cfg_if;



cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[cfg(test)]
        mod tests {
            use crate::tests::tests::get_test_server;
            use crate::auth::tests::tests::create_test_user;
            use crate::errors::*;
            use crate::bag::api::NewBagItem;

            use sqlx::SqlitePool;
            use sqlx::Row;
            use anyhow::Result;
            use axum_test::TestServer;
            use chrono::prelude::*;
            use leptos::logging;
            use futures::StreamExt;
            use crate::bag::model::*;


            #[sqlx::test]
            async fn test_item_e2e(pool: SqlitePool) -> Result<()> {
                let test_server = get_test_server(&pool).await?;
                let test_user = create_test_user(&test_server, None).await;

                let bi = BagItem {
                    added_by: test_user,
                    created_at: Utc::now(),
                    description: "Some description".into(),
                    name: "Some item".into(),
                    id: -1,
                    infinite: false,
                    quantity: 1,
                    size: ItemSize::Large
                };

                let new_bi = bi.insert(&pool).await?;
                assert_ne!(new_bi.id, -1);


                let by_id = BagItem::by_id(new_bi.id, &pool).await?;
                logging::log!("Fetched Bag item is {:?}", by_id);
                assert_eq!(by_id.is_some(), true);
                let mut by_id = by_id.unwrap();
                by_id.name = "new name".into();
                by_id.update(&pool).await?;

                let by_id2 = BagItem::by_id(by_id.id, &pool).await?;
                assert_eq!(by_id2.is_some(), true);
                let by_id2 = by_id2.unwrap();
                assert_eq!(by_id2.name, "new name");

                by_id2.delete(&pool).await?;
                let by_id2 = BagItem::by_id(by_id.id, &pool).await?;
                assert_eq!(by_id2.is_some(), false);
                Ok(())
            }

            #[sqlx::test]
            async fn test_filter_pagination(pool: SqlitePool) -> Result<()> {
                let test_server = get_test_server(&pool).await?;
                let test_user = create_test_user(&test_server, None).await;
                let test_user2 = create_test_user(&test_server, Some("scott2".into())).await;
                assert_ne!(test_user.id, test_user2.id);
                for _i in 0..10 {
                    let bi = BagItem {
                        added_by: test_user.clone(),
                        created_at: Utc::now(),
                        description: "Some description".into(),
                        name: "Some item".into(),
                        id: -1,
                        infinite: false,
                        quantity: 1,
                        size: ItemSize::Small
                    };

                    let new_bi = bi.insert(&pool).await?;
                    assert_ne!(new_bi.id, -1);
                }

                for _i in 0..10 {
                    let bi = BagItem {
                        added_by: test_user.clone(),
                        created_at: Utc::now(),
                        description: "Some description".into(),
                        name: "Some item".into(),
                        id: -1,
                        infinite: true,
                        quantity: 1,
                        size: ItemSize::Medium
                    };

                    let new_bi = bi.insert(&pool).await?;
                    assert_ne!(new_bi.id, -1);
                }

                for _i in 0..10 {
                    let bi = BagItem {
                        added_by: test_user2.clone(),
                        created_at: Utc::now(),
                        description: "Some description".into(),
                        name: "Some item".into(),
                        id: -1,
                        infinite: false,
                        quantity: 50,
                        size: ItemSize::Large
                    };

                    let new_bi = bi.insert(&pool).await?;
                    assert_ne!(new_bi.id, -1);
                }

                /*use sqlx::sqlite::*;
                use sqlx::Column;
                let res = sqlx::query_as::<_, BagItemDebug>("SELECT * FROM bagitems").fetch_all(&pool).await?;
                for r in res {
                    logging::log!("BagItem row is {:?}", r);
                }*/

                let user_filter = BagItemFilter {
                    added_by: Some(vec![test_user.id]),
                    page_size: Some(10),
                    ..Default::default()
                };
                let page = BagItem::filter(user_filter, &pool).await?;
                assert_eq!(page.total_results, 20);
                assert_eq!(page.items.len(), 10);
                assert_eq!(page.total_pages, 2);

                let size_filter = BagItemFilter {
                    size: Some(vec![ItemSize::Large.into()]),
                    ..Default::default()
                };
                let page = BagItem::filter(size_filter, &pool).await?;
                assert_eq!(page.total_results, 10);
                assert_eq!(page.items.len(), 10);
                assert_eq!(page.total_pages, 1);
                Ok(())
            }

            #[sqlx::test]
            async fn test_item_random(pool: SqlitePool) -> Result<()> {
                let test_server = get_test_server(&pool).await?;
                let test_user = create_test_user(&test_server, None).await;

                let bi = BagItem {
                    added_by: test_user,
                    created_at: Utc::now(),
                    description: "Some description".into(),
                    name: "Some item".into(),
                    id: -1,
                    infinite: false,
                    quantity: 1,
                    size: ItemSize::Large
                };

                let new_bi = bi.insert(&pool).await?;
                assert_ne!(new_bi.id, -1);

                let random_item = TakenBagItem::get_random(&pool).await?;
                assert_eq!(random_item.is_some(), true);
                let random_item = random_item.unwrap();
                assert_eq!(random_item.item.id, new_bi.id);

                let random_item2 = TakenBagItem::get_random(&pool).await?;
                assert_eq!(random_item2.is_some(), false);

                let for_item_vec = TakenBagItem::for_item(new_bi.id, &pool).await?;
                assert_eq!(for_item_vec.len(), 1);

                Ok(())
            }

            #[sqlx::test]
            async fn test_bagitem_api(pool: SqlitePool) -> Result<()> {
                let test_server = get_test_server(&pool).await?;

                let bi = NewBagItem {
                    description: "Some description".into(),
                    name: "Some item".into(),
                    infinite: false,
                    quantity: 1,
                    size: ItemSize::Large
                };

                let response = test_server.post("/api/create_bag_item")
                    .form(&bi)
                    .await;
                logging::log!("Response is {:?}", response);
                let res:RoadieResult<BagItem> = response.json();
                assert_eq!(res, Err(RoadieAppError::Unauthorized));

                //let test_user = create_test_user(&test_server, None).await;

                Ok(())
            }
        }
    }
}