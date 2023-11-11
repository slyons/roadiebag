use leptos::*;
use leptos_router::*;
use leptos_dom::*;
use crate::auth::frontend::AuthContext;

#[component]
pub fn Avatar() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");

    let user_first_letter = Signal::derive(move ||
        use_context::<AuthContext>().expect("Failed to get AuthContext")
            .user_first_letter());
    let show_avatar = Signal::derive(move || {
        let ac = use_context::<AuthContext>().expect("Failed to get AuthContext");
        if let Some(ur) = ac.user.get() {
            !ur.unwrap_or_default().anonymous
        } else {
            false
        }
    });

    view! {
        <Transition fallback=SuspenseContent>
            <Show when=show_avatar>
                <div className="dropdown dropdown-end ml-4">
                    <ActionForm action=auth_context.logout>
                        <div class="avatar placeholder">
                          <div class="bg-neutral-focus text-neutral-content rounded-full w-12">
                            <span>{user_first_letter}</span>
                          </div>
                          <button type="submit" class="btn btn-xs self-center">"Log Out"</button>
                        </div>
                    </ActionForm>
                </div>
            </Show>
        </Transition>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
          <div class="navbar-start">
            <div class="dropdown">
              <label tabindex="0" class="btn btn-ghost lg:hidden">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" /></svg>
              </label>
              <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
                <li><A exact=true href="/items">"Item List"</A></li>
                <li><A exact=true href="/">"Current Item"</A></li>
                <li><A exact=true href="/items/add">"Add Item"</A></li>
              </ul>
            </div>
            <A href="/" class="btn btn-ghost normal-case text-xl">"Thunk's Roadiebag"</A>
          </div>
          <div class="navbar-center hidden lg:flex">
            <ul class="menu menu-horizontal px-1">
                <li><A exact=true href="/items">"Item List"</A></li>
                <li><A exact=true href="/">"Current Item"</A></li>
                <li><A exact=true href="/items/add">"Add Item"</A></li>
            </ul>
          </div>
          <div class="navbar-end">
            <Avatar />
          </div>
        </div>
    }
}
#[component]
pub fn SuspenseContent() -> impl IntoView {
    view! {
       <div class="w-full h-screen text-gray-300 dark:text-gray-200 bg-base-100">
            Loading...
        </div>
    }
}

#[component]
pub fn PageContent(children: ChildrenFn) -> impl IntoView {
    use leptos::html::Main;
    let _main_ref: NodeRef<Main> = create_node_ref();
    //let location = use_location();

    let children = store_value(children);

    view! {
        <Header />
        <main class="pt-4 pr-4 pb-4 pl-4 mx-auto grid flex-col" node_ref=_main_ref>
                {children.with_value(|children| children())}
        </main>
    }
}

#[component]
pub fn Layout(children: ChildrenFn) -> impl IntoView {
    //let sidebar_signal = create_rw_signal(false);
    //provide_context(sidebar_signal);
    let children = store_value(children);
    view! {
        <PageContent>
            <Transition fallback=SuspenseContent>
                {children.with_value(|children| children())}
            </Transition>
        </PageContent>
    }
}