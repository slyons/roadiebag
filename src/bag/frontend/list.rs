use leptos_struct_table::*;
use leptos::*;
use leptos_router::*;
use std::cmp::min;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use serde_qs as qs;

use crate::bag::api::*;
use crate::bag::model::*;

#[derive(Clone, Copy)]
pub struct RoadiebagClassesPreset;

impl TableClassesProvider for RoadiebagClassesPreset {
    fn new() -> Self {
        Self
    }

    fn table(&self, classes: &str) -> String {
        format!(
            "{} {}",
            "w-full whitespace-no-wrap table-pin-cols table-zebra", classes
        )
    }

    fn head_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "",
            template_classes
        )
    }

    fn head_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn head_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, _row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if selected {
            "bg-base-200"
        } else {
            ""
        };

        format!(
            "{} {} {}",
            "", bg_color, template_classes
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable, classes_provider="RoadiebagClassesPreset")]
pub struct ListItem {
    #[table(key, skip)]
    pub id: i64,
    pub name: String,
    pub description: String,
    pub quantity: i32,
    pub size: String,
    pub infinite: bool,
    pub added_by: String,

    #[table(renderer=EditLinkCellRenderer)]
    pub edit_link: FieldGetter<String>
}

impl ListItem {
    pub fn edit_link(&self) -> String {
        format!("/items/edit/{}", self.id.to_string())
    }
}

#[component]
fn EditLinkCellRenderer<F>(
    #[prop(into)] class: MaybeSignal<String>,
    #[prop(into)] value: MaybeSignal<String>,
    on_change: F,
    index: usize,
) -> impl IntoView
    where
        F: Fn(String) + 'static {
    view! {
        <td class=class>
            <A href=value>"Edit"</A>
        </td>
    }
}

impl From<BagItem> for ListItem {
    fn from(value: BagItem) -> Self {
        ListItem {
            id: value.id.clone(),
            added_by: value.added_by.username,
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            size: value.size.to_string(),
            infinite: value.infinite,
            edit_link: Default::default()
        }
    }
}

#[component]
pub fn ItemListPagination(current_page:Resource<BagItemFilter, Option<BagItemPage>>) -> impl IntoView {

    let page_max = Signal::derive(move || {
        let page = use_context::<Resource<BagItemFilter, Option<BagItemPage>>>().expect("Unable to fetch page");

        let page_num = if let Some(Some(pn)) = page() {
            pn.page_num
        } else {
            1
        };

        let total_pages = if let Some(Some(page)) = current_page() {
            page.total_pages
        } else {
            1
        };

        min(page_num +2, total_pages)
    });

    let pages = Signal::derive(move || {
        let query = use_context::<Memo<BagItemFilter>>().expect("Unable to fetch query");
        let page = use_context::<Resource<BagItemFilter, Option<BagItemPage>>>().expect("Unable to fetch page");

        let page_num = if let Some(Some(pn)) = page() {
            pn.page_num
        } else {
            1
        };

        let page_min = page_num
            .checked_sub(2)
            .unwrap_or(page_num);

        let total_pages = if let Some(Some(page)) = current_page() {
            page.total_pages
        } else {
            1
        };

        let page_max = min(page_num +2, total_pages);

        let mut pages = vec![];
        for i in page_min..=page_max {
            let new_qs = qs::to_string(&query().with_page(i)).expect("Couldn't serialize query string");
            let class = if page_num == i {
                "join-item btn btn-disable"
            } else {
                "join-item btn"
            };
            pages.push((
                i,
                format!("/list?{}", new_qs),
                class
            ));
        }
        pages
    });

    view! {
        <div class="pt-4 w-full flex justify-center">
            <div class="join">

                <a class="join-item btn" exact=true href="/list?page_num=1">"«"</a>
                <For each=pages
                    key=|link| link.0
                    children=move |(id, query, class)| {
                        view! {
                            <a class={class} href=query>{id}</a>
                        }
                    }
                />
                <a class="join-item btn" exact=true href=move|| {format!("/list?page_num={}", page_max())}>"»"</a>
            </div>
        </div>
    }
}

#[component]
pub fn ItemList() -> impl IntoView {
    logging::log!("Item list time");
    let location = use_location();
    let query = create_memo(move |_| {
        location.search.with(|m| {
            let qs_config = qs::Config::new(0, false);
            let bif = qs_config.deserialize_str::<BagItemFilter>(m).unwrap_or(BagItemFilter {
                page_num: Some(1),
                page_size: Some(50),
                ..Default::default()
            });
            logging::log!("Query string is {} bif is {:?}", m, bif);
            bif
        })
    });
    provide_context(query);
    //let query = use_query::<BagItemFilter>();
    let page = create_resource(
        query,
        |filter| async move {
            match list_bag_items(Some(filter)).await {
                Ok(Ok(page)) => {
                    Some(page)
                },
                _ => None
            }
        }
    );
    provide_context(page);

    let page_items = create_rw_signal(Vec::<ListItem>::new());

    create_effect(move |_| {
        if let Some(Some(pg)) = page() {
            let mut item_vec = Vec::<ListItem>::new();
            for bi in pg.items.into_iter() {
                item_vec.push(bi.into());
            }
            page_items.set(item_vec);
        }
    });

    view! {
        <div class="mt-0 mr-8 mb-0 ml-0 w-full h-full flex flex-col bg-base-100 shadow-xl">
            <div class="h-full w-full pb-6 bg-base-100" >
                <div class="overflow-x-auto">
                    <ListItemTable items=page_items />
                </div>
                <ItemListPagination current_page=page/>
            </div>
        </div>
    }
}