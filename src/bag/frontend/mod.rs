mod list;
mod addedit;
mod current;

use leptos::*;
use leptos_router::*;
use crate::auth::frontend::{AuthContext};
use crate::bag::api::{get_bag_item, BagItemForm};

#[component]
pub fn BagRoutesOutlet() -> impl IntoView {
    view! {
        <Outlet />
    }
}



#[component(transparent)]
pub fn BagRoutes() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    let is_not_anonymous = Signal::derive(|| {
        let ac = use_context::<AuthContext>().expect("Failed to get AuthContext");
        let anon = if let Some(ur) = ac.user.get() {
            !ur.unwrap_or_default().anonymous
        } else {
            false
        };
        logging::log!("Is User not anonymous? {}", anon);
        anon
    });

    view! {
        <ProtectedRoute
            path="/"
            view=BagRoutesOutlet
            condition=is_not_anonymous
            redirect_path="/auth"
        >
            <ProtectedRoute path=""
                view=current::CurrentItem
                condition=is_not_anonymous
                redirect_path="/auth"
            />
            <ProtectedRoute path="items"
                view=list::ItemList
                condition=is_not_anonymous
                redirect_path="/auth"
            />
            <ProtectedRoute path="items/add"
                view=addedit::AddEditItem
                condition=is_not_anonymous
                redirect_path="/auth"
            />
            <ProtectedRoute path="items/edit/:id"
                view=addedit::AddEditItem
                condition=is_not_anonymous
                redirect_path="/auth"
            />
        </ProtectedRoute>
    }
}