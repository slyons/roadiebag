use crate::auth::frontend::AuthContext;
use leptos::*;
use leptos_dom::*;
use leptos_router::*;
use crate::auth::api::*;

#[component]
pub fn Avatar() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let logout = create_server_action::<LogoutAPI>();
    let is_authed = auth_context.auth_signal();

    let user_first_letter = Signal::derive(move || {
        use_context::<AuthContext>()
            .expect("Failed to get AuthContext")
            .user_first_letter()
    });
    let show_avatar = Signal::derive(move || {
        let x = is_authed();

        leptos::logging::log!("I'm the avatar {}", x);
        x
    });

    create_effect(move |_| {
        if let Some(Ok(_)) = logout.value().get() {
            use_navigate()("/auth", Default::default())
        }
    });

    view! {
        <Transition fallback=SuspenseContent>
            <Show when=show_avatar>
                <div className="dropdown dropdown-end ml-4">
                    <ActionForm action=logout>
                        <div class="avatar placeholder">
                            <div class="bg-neutral-focus text-neutral-content rounded-full w-12">
                                <span>{user_first_letter}</span>
                            </div>
                            <button type="submit" class="btn btn-xs self-center">
                                "Log Out"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </Show>
        </Transition>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let is_authed = Signal::derive(move || auth_context.auth_signal().get());
    view! {
        <div class="navbar bg-base-100">
            <Transition fallback=SuspenseContent>
                // 2
                <div class="navbar-start">
                    // 3
                    <div class="dropdown">
                        <label tabindex="0" class="btn btn-ghost lg:hidden">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h8m-8 6h16"
                                ></path>
                            </svg>
                        </label>
                        <ul
                            tabindex="0"
                            class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                        >

                            <Show when=is_authed>
                                <li>
                                    <A exact=true href="/items">
                                        "Item List"
                                    </A>
                                </li>
                                <li>
                                    <A exact=true href="/">
                                        "Current Item"
                                    </A>
                                </li>
                                <li>
                                    <A exact=true href="/items/add">
                                        "Add Item"
                                    </A>
                                </li>
                            </Show>

                        </ul>
                    // 3
                    </div>
                    <A href="/" class="btn btn-ghost normal-case text-xl">
                        "Thunk's Roadiebag"
                    </A>
                // 2
                </div>
                // 2
                <div class="navbar-center hidden lg:flex">
                    <ul class="menu menu-horizontal px-1">
                        <Show when=is_authed>
                            <li>
                                <A exact=true href="/items">
                                    "Item List"
                                </A>
                            </li>
                            <li>
                                <A exact=true href="/">
                                    "Current Item"
                                </A>
                            </li>
                            <li>
                                <A exact=true href="/items/add">
                                    "Add Item"
                                </A>
                            </li>
                        </Show>
                    </ul>
                // 2
                </div>
                // 2
                <div class="navbar-end">
                    <Avatar/>
                // 2
                </div>
            </Transition>
        </div>
    }
}
#[component]
pub fn SuspenseContent() -> impl IntoView {
    view! { <div class="w-full h-screen text-gray-300 dark:text-gray-200 bg-base-100">Loading...</div> }
}

#[component]
pub fn PageContent(children: ChildrenFn) -> impl IntoView {
    use leptos::html::Main;
    let _main_ref: NodeRef<Main> = create_node_ref();
    //let location = use_location();

    let children = store_value(children);

    view! {
        <Header/>
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
    view! { <PageContent>{children.with_value(|children| children())}</PageContent> }
}
