mod list;
mod addedit;

use leptos::*;
use leptos_router::{Route, Outlet};
use crate::common::components::layout::Layout;

#[component]
pub fn BagItemLayout() -> impl IntoView {
    view! {
        <Layout>
            <Outlet />
        </Layout>
    }
}

#[component(transparent)]
pub fn BagRoutes() -> impl IntoView {

    view! {
        <Route path="/" view=BagItemLayout>
            <Route path="" view=list::ItemList />
            <Route path="items/add" view=addedit::AddEditItem />
            <Route path="items/edit/:id" view=addedit::AddEditItem />
        </Route>
    }
}