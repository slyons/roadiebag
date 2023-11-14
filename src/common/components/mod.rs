pub(crate) mod input;
pub(crate) mod layout;
use std::str::FromStr;
use strum::*;

use leptos::*;
use leptos_use::is_some;

#[derive(Debug, Copy, Clone, EnumString)]
pub enum AlertType {
    None,
    Info,
    Success,
    Warning,
    Error
}
#[component]
pub fn Alert(
            alert_type: String,
            msg: Signal<Option<String>>,
) -> impl IntoView {
    let alert_type = Signal::derive(move || alert_type.clone());
    let symbol = Signal::derive(move || { match AlertType::from_str(alert_type().as_ref()).expect("Can't parse alert type") {
        AlertType::None => view! { <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-info shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg> },
        AlertType::Info => view! { <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>},
        AlertType::Success => view! { <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>},
        AlertType::Warning => view! { <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg> },
        AlertType::Error => view! { <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg> }
    }});

    let alert_class = Signal::derive(move || { match AlertType::from_str(alert_type().as_ref() ).expect("Can't parse alert type") {
        AlertType::None => "mt-4 alert",
        AlertType::Info => "mt-4 alert alert-info",
        AlertType::Success => "mt-4 alert alert-success",
        AlertType::Warning => "mt-4 alert alert-warning",
        AlertType::Error => "mt-4 alert alert-error"
    }});

    //let alert_class = store_value(alert_class);
    //let symbol = store_value(symbol);
    //let msg2 = Signal::derive(move || msg());
    let show_alert = is_some(msg);
    //let msg = Signal::derive(|| msg());

    view! {
        <Show when=show_alert>
            <div class=move || alert_class()>
                {symbol()}
                <span>{msg()}</span>
            </div>
        </Show>
    }
}