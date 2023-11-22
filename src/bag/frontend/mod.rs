mod addedit;
mod current;
mod list;

use crate::auth::frontend::AuthContext;
use leptos::*;
use leptos_router::*;

#[component]
pub fn BagRoutesOutlet() -> impl IntoView {
    view! { <Outlet/> }
}

#[component(transparent)]
pub fn BagRoutes() -> impl IntoView {

    let is_authed = Signal::derive(|| {
        let ac = use_context::<AuthContext>().expect("Failed to get AuthContext");
        if let Some(ur) = ac.user.get() {
            !ur.unwrap_or_default().anonymous
        } else {
            false
        }
    });

    view! {
        <ProtectedRoute path="/" view=BagRoutesOutlet condition=is_authed redirect_path="/auth">
            <ProtectedRoute
                path=""
                view=current::CurrentItem
                condition=is_authed
                redirect_path="/auth"
            />
            <ProtectedRoute
                path="items"
                view=list::ItemList
                condition=is_authed
                redirect_path="/auth"
            />
            <ProtectedRoute
                path="items/add"
                view=addedit::AddEditItem
                condition=is_authed
                redirect_path="/auth"
            />
            <ProtectedRoute
                path="items/edit/:id"
                view=addedit::AddEditItem
                condition=is_authed
                redirect_path="/auth"
            />
        </ProtectedRoute>
    }
}
