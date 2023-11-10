use leptos::*;
use leptos::ev::SubmitEvent;
use leptos_router::*;
use crate::common::components::input::*;
use crate::common::components::{AlertType, Alert};
use std::str::FromStr;
use strum::*;

use crate::bag::api::*;
use crate::bag::model::*;

#[derive(Params, PartialOrd, PartialEq, Debug, Copy, Clone)]
struct AddEditParams {
    id: Option<i64>
}

#[component]
pub fn AddEditItem() -> impl IntoView {
    let params = use_params::<AddEditParams>();
    let state = create_rw_signal(BagItemForm::default());

    create_effect(move |_| {
        logging::log!("Params is {:?}", params.get());
    });

    // TODO: Have better handling for items that 404
    let item_loader = create_resource(
        params,
        move |p| async move{
            if let Ok(p) = p {
                if let Some(id) = p.id {
                    let bif = get_bag_item(id).await
                                .expect("Server error")
                                .expect("Item not found")
                                .into();
                    state.set(bif);
                } else {
                    state.set(BagItemForm::default());
                }
            } else {
                logging::error!("Unable to parse params");
                use_navigate()("/", Default::default());
            }
        }
    );

    provide_context(state);
    view! {
        <ItemForm />
    }
}

/*#[component]
pub fn EditItem() -> impl IntoView {
    let params = use_params::<AddEditParams>();
}*/

#[component]
pub fn ItemForm() -> impl IntoView {
    let state = expect_context::<RwSignal<BagItemForm>>();
    let (submit_error, set_submit_error) = create_signal(None);

    let item_id = Signal::derive(move || {
        state.with(|s| s.id.unwrap_or(-1))
    });

    let (name, set_name) = create_slice(
        state,
        |state| state.name.clone(),
        |state, n| state.name = n
    );

    let (desc, set_desc) = create_slice(
        state,
        |state| state.description.clone(),
        |state, n| state.description = n
    );

    let (quantity, set_quantity) = create_slice(
        state,
        |state| state.quantity.to_string(),
        |state, n:String| {
            if !n.is_empty() {
                state.quantity=n.parse().expect("Unable to parse integer")
            }
        }
    );

    let (size, set_size) = create_slice(
        state,
        |state| {
            if state.size == ItemSize::Unknown {
                None
            } else {
                Some(state.size.to_string())
            }
        },
        |state, n:Option<String>| {
            if let Some(i) = n {
                state.size = ItemSize::from_str(&i).expect("Item size parse failure");
            }
        }
    );
    let options = ItemSize::iter()
        .map(|variant| (variant.to_string(), variant.to_string()))
        .collect::<Vec<(String, String)>>();

    let (infinite, set_infinite) = create_slice(
        state,
        |state| state.infinite,
        |state, n| state.infinite = n
    );


    let submit_text = Signal::derive(move || {
        if state().id.is_some() {
            "Update".to_string()
        } else {
            "Create".to_string()
        }
    });

    let submit_action = create_action(move |input: &BagItemForm| {
        let input = input.to_owned();
        async move {
            if input.id.is_some() {
                let res = update_bag_item(input).await;
                match res {
                    Ok(Ok(_)) => set_submit_error(None),
                    Ok(Err(e)) => set_submit_error(Some(e.to_string())),
                    Err(e) => set_submit_error(Some(e.to_string()))
                }
            } else {
                let res = create_bag_item(input).await;
                match res {
                    Ok(Ok(_)) => set_submit_error(None),
                    Ok(Err(e)) => set_submit_error(Some(e.to_string())),
                    Err(e) => set_submit_error(Some(e.to_string()))
                }
            }
            ()
        }
    });

    // TODO: Create spinner on action
    // TODO: Disable submit button on submit
    // TODO: Show spinner on initial load

    //Reset the form on successful submit
    create_effect(move |_| {
        submit_action.value().track();
        if submit_error.get_untracked().is_none() {
            state.set(BagItemForm::default());
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_submit_error(None);
        if name.get().trim().len() == 0 {
            set_submit_error(Some("You must specify an item name".into()));
        }
        if quantity.get().trim().len() == 0 {
            set_submit_error(Some("You must specify an item name".into()));

        } else {
            let q = quantity.get();
            let v = q.parse::<i32>();
            match v {
                Err(e) => set_submit_error(Some(e.to_string())),
                Ok(quan) => if quan <= 0 {
                    set_submit_error(Some("Quantity must be > 0".into()))
                }
            };
        }
        if size.get().is_none() {
            set_submit_error(Some("Item size must be specified".into()));
        }

        if submit_error.get().is_none() {
            submit_action.dispatch(state.get());
        }
    };

    view! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="bg-base-100 rounded-xl">
                    <div class="py-24 px-10 w-full">
                        <form on:submit=on_submit>
                            <h2 class="text-2xl font-semibold mb-2 text-center">{format!("{} Item", submit_text())}</h2>
                            <InputControlled
                                input_type="text"
                                field_name="name"
                                field_label="Name"
                                container_style="mb-4"
                                value=name
                                set_value=set_name/>
                            <TextareaControlled
                                input_type="textarea"
                                field_name="description"
                                field_label="Item Description"
                                container_style="mb-4"
                                value=desc
                                set_value=set_desc/>
                            <InputControlled
                                input_type="number"
                                field_name="quantity"
                                field_label="Quantity"
                                container_style="mb-4"
                                value=quantity
                                set_value=set_quantity/>
                            <SelectControlled
                                field_label="Item Size"
                                field_name="itemsize"
                                container_style="mb-4"
                                options=options
                                selected=size
                                set_selected=set_size />
                            <CheckboxControlled
                                field_name="infinite"
                                field_label="Infinite"
                                container_style="mb-4"
                                input_style="toggle "
                                value=infinite
                                set_value=set_infinite />
                            <Alert alert_type=AlertType::Error msg=submit_error />
                            <button type="submit" class="btn mt-2 w-full btn-primary">{submit_text()}</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}