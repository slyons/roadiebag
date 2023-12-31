use crate::auth::frontend::Auth;
use crate::auth::provide_auth;
use crate::bag::frontend::BagRoutes;
use crate::common::components::layout::*;
use crate::error_template::ErrorTemplate;
use crate::errors::RoadieAppError;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::panic;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    view! {
        <Html attr:data-theme="cupcake"/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="https://cdn.jsdelivr.net/npm/daisyui@3.9.4/dist/full.css"/>
        // <Stylesheet id="leptos" href="/pkg/roadiebag.css"/>

        // sets the document title
        <Title text="Welcome to Thunk's Roadie Bag"/>

        <div id="root">
            // content for this welcome page

            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(RoadieAppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>

                {
                    provide_auth();
                }
                <Routes>
                    <Route path="/" view=RoadieBagPage>
                        <Auth/>
                        <BagRoutes/>
                    </Route>
                </Routes>
            </Router>
        </div>

        <Script src="https://cdn.tailwindcss.com"/>
    }
}

/// Renders the home page of your application.
#[component(transparent)]
fn RoadieBagPage() -> impl IntoView {
    view! {
        <Layout>
            <Outlet/>
        </Layout>
    }
}
