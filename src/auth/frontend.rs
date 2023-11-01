use leptos::*;


use leptos_router::ActionForm;
use model::User;
use crate::auth::model;
use crate::auth::api::*;

use crate::errors::RoadieResult;

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<LoginAPI, Result<RoadieResult<User>, ServerFnError>>,
    pub logout: Action<LogoutAPI, Result<(), ServerFnError>>,
    pub signup: Action<SignupAPI, Result<RoadieResult<()>, ServerFnError>>,
    pub user: Resource<(usize, usize, usize), Result<User, ServerFnError>>
}

pub fn provide_auth() {
    let login = create_server_action::<LoginAPI>();
    let logout = create_server_action::<LogoutAPI>();
    let signup = create_server_action::<SignupAPI>();
    let user = create_resource(
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup.version().get()
                )
        },
        |_| async move  {
            get_user().await
        }
    );
    provide_context(AuthContext {
        user,
        signup,
        logout,
        login
    });
}

#[component]
pub fn CSignup() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
        <div>
            <ActionForm action=auth_context.signup>
                <h1>"Sign up"</h1>
                <label>
                    "User ID:"
                    <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
                </label>
                <br/>
                <label>
                    "Password:"
                    <input type="password" placeholder="Password" name="password" class="auth-input" />
                </label>
                <br/>
                <label>
                    "Confirm Password:"
                    <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
                </label>
                <br/>
                <label>
                    "Remember me?"
                    <input type="checkbox" name="remember" class="auth-input" />
                </label>

                <br/>
                <button type="submit" class="button">"Sign Up"</button>

            </ActionForm>
        </div>
    }
}

#[component]
pub fn CLogin(
    show_signup: RwSignal<bool>
) -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    view! {
        <div>
            <ActionForm action=auth_context.login>
                <h1>"Log In"</h1>
                <label>
                    "User ID:"
                    <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
                </label>
                <br/>
                <label>
                    "Password:"
                    <input type="password" placeholder="Password" name="password" class="auth-input" />
                </label>
                <br/>
                <label>
                    <input type="checkbox" name="remember" class="auth-input" />
                    "Remember me?"
                </label>
                <br/>
                <button type="submit" class="button">"Log In"</button>
            </ActionForm>
            <button class="button" on:click=move |_| show_signup.set(true)>"Sign up"</button>
        </div>
    }
}

#[component]
pub fn AuthCard() -> impl IntoView {
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    let show_signup = create_rw_signal(false);

    create_effect(move |_| {
        auth_context.signup.version();
        show_signup.set(false);
    });

    let is_logged_in = Signal::derive(move || {
        match auth_context.user.get() {
            Some(Ok(u)) => !u.anonymous,
            _ => false
        }
    });
    let username = Signal::derive(move || {
        match auth_context.user.get() {
            Some(Ok(u)) if !u.anonymous => u.username,
            _ => "Anonymous".to_string()
        }
    });

    view! {
        <div id="loginbox">
            <Transition>
                <Show when=is_logged_in>
                    <div>You are logged in as {username()}</div>
                    <ActionForm action=auth_context.logout>
                        <button type="submit" class="button">"Log Out"</button>
                    </ActionForm>
                </Show>
                <Show when=move || !is_logged_in() && !show_signup()>
                    <CLogin show_signup=show_signup />
                </Show>
                <Show when=move || !is_logged_in() && show_signup()>
                    <CSignup />
                </Show>
            </Transition>
        </div>
    }
}
