use leptos::*;
use leptos_router::*;
use crate::auth::frontend::AuthContext;
use serde_qs as qs;
use super::api::*;
use super::model::*;

#[component]
pub fn ItemListRow(item: BagItem) -> impl IntoView {
    let _auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
        <tr>
            <td>{item.id}</td>
            <td>{item.name}</td>
            <td>{item.description}</td>
            <td>{item.quantity}</td>
            <td>{item.size.to_string()}</td>
            <td>{item.infinite}</td>
            <td>{ move || format!("{}", item.created_at.format("%Y-%m-%d %H:%M:%S"))}</td>
        </tr>
    }
}

#[component]
pub fn ItemListTable(
    items: Signal<Vec<BagItem>>
) -> impl IntoView {
    view! {
        <div>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Description</th>
                        <th>Qty</th>
                        <th>Size</th>
                        <th>Inf</th>
                        <th>Creation Time</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=items
                        key=|item| item.id
                        children=move |item| view! { <ItemListRow item=item />}
                    />
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ItemList() -> impl IntoView {
    let location = use_location();
    let query = create_memo(move |_| {
        location.search.with(|m| qs::from_str::<BagItemFilter>(m).unwrap())
    });
    //let query = use_query::<BagItemFilter>();
    let page = create_resource(
        query,
        |filter| async move {
                match list_bag_items(Some(filter)).await {
                    Ok(Ok(page)) => Some(page),
                    _ => None
                }
        }
    );
    let page_items = Signal::derive(move || {
        page.read()
            .map(|op| op.map(|pg| pg.items).unwrap_or(vec![]))
            .unwrap_or(vec![])
    });

    view! {
        <ItemListTable items=page_items />
    }
}