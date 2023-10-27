use crate::error_template::{ErrorTemplate};
use crate::errors::RoadieAppError;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::auth::frontend::{AuthContext, AuthCard, CLogin, CSignup};
use crate::auth::provide_auth;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();

    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    create_effect(move |_x| {

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
    });

    let is_anonymous = Signal::derive(move || {
        match auth_context.user.read() {
            Some(Ok(u)) => u.anonymous,
            _ => true
        }
    });

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/roadiebag.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <AuthCard />
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(RoadieAppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/auth" view=CLogin/>
                    <Route path="/signup" view=CSignup />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
