use leptos::*;

use crate::auth::api::*;
use crate::auth::model;
use crate::common::components::input::*;
use crate::common::components::Alert;
use crate::errors::RoadieResult;
use leptos_router::*;
use model::User;

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<LoginAPI, Result<RoadieResult<User>, ServerFnError>>,
    pub logout: Action<LogoutAPI, Result<(), ServerFnError>>,
    pub signup: Action<SignupAPI, Result<RoadieResult<()>, ServerFnError>>,
    pub user: Resource<(usize, usize, usize, ()), Result<User, ServerFnError>>,
}

impl AuthContext {
    pub fn is_anonymous(&self) -> bool {
        match (self.user)() {
            Some(Ok(u)) => u.anonymous,
            _ => true,
        }
    }

    pub fn user_first_letter(&self) -> String {
        let username = match (self.user)() {
            Some(Ok(u)) if !u.username.is_empty() => u.username,
            _ => "Anonymous".to_string(),
        };

        username
            .chars()
            .nth(0)
            .unwrap()
            .to_ascii_uppercase()
            .to_string()
    }
}

pub fn provide_auth() {
    let location = use_location();
    let login = create_server_action::<LoginAPI>();
    let logout = create_server_action::<LogoutAPI>();
    let signup = create_server_action::<SignupAPI>();
    let user = create_resource(
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup.version().get(),
                location.state.track(),
            )
        },
        |_| async move { get_user().await },
    );
    provide_context(AuthContext {
        user,
        signup,
        logout,
        login,
    });
}

#[component]
pub fn CSignup() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let (signup_error, set_signup_error) = create_signal(None);

    create_effect(move |_| match auth_context.signup.value().get() {
        Some(Ok(Err(e))) => set_signup_error(Some(e.to_string())),
        _ => set_signup_error(None),
    });

    create_effect(move |_| {
        if let Some(Ok(Ok(_))) = auth_context.signup.value().get() {
            use_navigate()("/auth", Default::default())
        }
    });
    view! {
        <h2 class="text-2xl font-semibold mb-2 text-center">"Register"</h2>
        <ActionForm action=auth_context.signup>

            <div class="mb-4">
                <InputText
                    field_name="username"
                    input_type="emailId"
                    container_style="mt-4"
                    field_label="Username"
                />
                <InputText
                    field_name="password"
                    input_type="password"
                    container_style="mt-4"
                    field_label="Password"
                />
                <InputText
                    field_name="password_confirmation"
                    input_type="password"
                    container_style="mt-4"
                    field_label="Password Confirmation"
                />
            </div>
            <Alert alert_type="Error".into() msg=signup_error.into_signal()/>
            <button type="submit" class="btn mt-2 w-full btn-primary">
                "Register"
            </button>

            <div class="text-center mt-4">
                "Already have an account?" <A href="/auth">
                    <span class="inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200 px-3">
                        "Login"
                    </span>
                </A>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn CLogin() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let (auth_error, set_auth_error) = create_signal(None);

    create_effect(move |_| match auth_context.login.value().get() {
        Some(Ok(Err(e))) => set_auth_error(Some(e.to_string())),
        _ => set_auth_error(None),
    });

    create_effect(move |_| {
        if let Some(Ok(Ok(_))) = auth_context.login.value().get() {
            use_navigate()("/", Default::default());
        }
    });

    view! {
        <h2 class="text-2xl font-semibold mb-2 text-center">"Login"</h2>
        <ActionForm action=auth_context.login>
            <div class="mb-4">
                <InputText
                    field_name="username"
                    input_type="emailId"
                    container_style="mt-4"
                    field_label="Username"
                />
                <InputText
                    field_name="password"
                    input_type="password"
                    container_style="mt-4"
                    field_label="Password"
                />
                <div class="form-control mt-4">
                    <label class="label justify-center">
                        <span class="label-text text-xs self-center">"Remember me?"</span>
                    </label>
                    <input type="checkbox" name="remember" class="checkbox input-xs self-center"/>
                </div>
            </div>
            <Alert alert_type="Error".into() msg=auth_error.into_signal()/>

            <button type="submit" class="btn mt-2 w-full btn-primary">
                "Login"
            </button>
            <div class="text-center mt-4">
                "Don't have an account yet?" <A href="/auth/register">
                    <span class="  inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200 px-3">
                        "Register"
                    </span>
                </A>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn AuthWrapper() -> impl IntoView {
    logging::log!("AuthWrapper");
    view! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="bg-base-100 rounded-xl">
                    <div class="py-24 px-10 w-full">
                        <Outlet/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component(transparent)]
pub fn Auth() -> impl IntoView {
    view! {
        <Route path="/auth" view=AuthWrapper>
            <Route path="/register" view=CSignup/>
            <Route path="" view=CLogin/>
        </Route>
    }
}
