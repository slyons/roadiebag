use crate::error_template::{ErrorTemplate};
use crate::errors::RoadieAppError;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::frontend::{AuthContext, Auth};
use crate::bag::frontend::BagRoutes;
use crate::common::components::layout::SuspenseContent;
use crate::auth::provide_auth;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");


    /*create_effect(move |_x| {

        match auth_context.login.value().get() {
            Some(Ok(Ok(_))) => use_navigate()("/", Default::default()),
            _ => ()
        }
    });
    create_effect(move |_x| {
        let x = auth_context.signup.value().get();
        logging::log!("Signup is {:?}", x);
        match x {
            Some(Ok(Ok(_))) => use_navigate()("/auth", Default::default()),
            _ => ()
        }
    });*/

    let _is_anonymous = Signal::derive(move || {
        match (auth_context.user)() {
            Some(Ok(u)) => u.anonymous,
            _ => true
        }
    });



    view! {
        <Html attr:data-theme="cupcake" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="https://cdn.jsdelivr.net/npm/daisyui@3.9.4/dist/full.css"/>
        //<Stylesheet id="leptos" href="/pkg/roadiebag.css"/>

        // sets the document title
        <Title text="Welcome to Thunk's Roadie Bag"/>

        <div id="root">
        // content for this welcome page
            <Transition fallback=SuspenseContent>
                <Router fallback=|| {
                        let mut outside_errors = Errors::default();
                        outside_errors.insert_with_default_key(RoadieAppError::NotFound);
                        view! {
                            <ErrorTemplate outside_errors/>
                        }
                        .into_view()
                    }>
                    //<main>
                        <Routes>
                            <Route path="/" view=RoadieBagPage>
                                <Auth />
                                <BagRoutes />
                            </Route>
                        </Routes>
                    //</main>
                </Router>
            </Transition>
        </div>

        <Script src="https://cdn.tailwindcss.com"></Script>
    }
}

/// Renders the home page of your application.
#[component(transparent)]
fn RoadieBagPage() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let location = use_location();

    create_effect(move |_| {
        if let Some(user_res) = auth_context.user.get() {
            match user_res {
                Ok(u) if u.anonymous => {
                    let path:String = location.pathname.get();
                    if !path.starts_with("/auth") {
                        logging::log!("Redirecting user to auth");
                        use_navigate()("/auth", Default::default());
                    }
                },
                Ok(_u) => {
                    let path:String = location.pathname.get();
                    if path.starts_with("/auth") {
                        logging::log!("Redirecting user to item list");
                        use_navigate()("/", Default::default());
                    }
                }
                _ => ()
            }

        }
    });

    view! {
        <Outlet />
    }
}

