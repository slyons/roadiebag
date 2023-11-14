use leptos::*;
use leptos::ev::SubmitEvent;
use leptos_router::*;
use crate::common::components::input::*;
use crate::common::components::{AlertType, Alert};
use crate::common::create_resource_slice;
use std::str::FromStr;
use std::collections::HashMap;
use strum::*;

use crate::bag::api::*;
use crate::bag::model::*;
use crate::errors::{RoadieAppError, RoadieResult};
use crate::error_template::ErrorTemplate;

#[derive(Params, Default, PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct AddEditParams {
    id: Option<i64>
}

#[component]
pub fn AddEditItem() -> impl IntoView {
    view! {
        <ItemForm editable=true/>
    }
}

#[component]
pub fn ItemForm(editable: bool) -> impl IntoView {
    let params = use_params::<AddEditParams>();
    let (submit_error, set_submit_error) =
        create_signal(HashMap::new());
    let action = create_server_action::<CreateUpdateBagItem>();

    let item_loader = create_resource(
        move || params.get(),
        move |p| async move{
            if let Ok(p) = p {
                if let Some(id) = p.id {
                    match get_bag_item(id).await {
                        Ok(Err(RoadieAppError::MultipleErrors(e))) => set_submit_error(e),
                        Err(e) => {
                            set_submit_error.update(|m| {m.insert("other".to_string(), e.to_string());});
                        },
                        Ok(Err(e)) => {
                            set_submit_error.update(|m| {m.insert("other".to_string(), e.to_string());});
                        },
                        Ok(Ok(bif)) => {
                            let bif:BagItemForm = bif.into();
                            logging::log!("Loaded item {:?}", bif);
                            action.value().set(Some(Ok(Ok(bif))));
                        }
                    };
                } else {
                    logging::log!("Loading default item");
                    action.value().try_set(Some(Ok(Ok(BagItemForm::default()))));

                }
            } else {
                logging::error!("Unable to parse params");
                use_navigate()("/items", Default::default());
            }
        }
    );

    let submit_text = move || {
        action.pending().track();
        if let Some(Ok(Ok(r))) = action.value().get() {
            if r.id == -1 {
                "Create Item".to_string()
            } else {
                "Update Item".to_string()
            }
        } else {
            "".to_string()
        }
    };

    /*let (item_id, _set_item_id) = create_slice(
        current_item,
        |ci| ci.map(|c| c.id),
        |_ci, _v| ()
    );

    let (name, set_name) = create_slice(
        current_item,
        |ci| ci.map(|c| c.name),

        |ci, n| {
            if let Ok(ci) = ci {
                ci.name = n;
            }
        }
    );

    let (desc, set_desc) = create_slice(
        current_item,
        |ci| ci.map(|c| c.description),
        |ci, n| {
            if let Ok(ci) = ci {
                ci.description = n;
            }
        }
    );

    let (quantity, set_quantity) = create_slice(
        current_item,
        |ci| ci.map(|c| c.quantity),
        |ci, n:String| {
            let to_int = n.parse();
            match (ci, to_int) {
                (Ok(ci), Ok(qnt)) => ci.quantity = qnt,
                (_, Err(e)) => set_submit_error(Some(RoadieAppError::ItemQntGtZero.to_string())),
                _ => ()
            };
        }
    );

    let (size, set_size) = create_slice(
        current_item,
        |ci| ci.map(|c| c.size),
        |ci, n:String| {
            let to_size = ItemSize::from_str(&i);
            match (ci, to_size) {
                (Ok(ci), Ok(size)) => ci.size = size,
                (_, Err(e)) => set_submit_error(Some(RoadieAppError::ItemSizeMustBeSet.to_string()))
            }
        }
    );

    let (infinite, set_infinite) = create_slice(
        current_item,
        |ci| ci.map(|c| c.infinite),
        |ci, n:bool| {
            if let Ok(ci) = ci {
                ci.infinite = n;
            }
        }
    );*/

    let size_options = ItemSize::iter()
        .filter(|v| v != &ItemSize::Unknown)
        .map(|variant| (variant.to_string(), variant.to_string()))
        .collect::<Vec<(String, String)>>();

    /*let submit_action = create_action(move |input: &BagItemForm| {
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
    });*/

    // TODO: Create spinner on action
    // TODO: Disable submit button on submit
    // TODO: Show spinner on initial load

    //Reset the form on successful submit
    /*create_effect(move |_| {
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
            set_submit_error(Some("You must specify a quantity".into()));

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
            submit_action.dispatch(state.get().unwrap());
        }
    };*/
    let result = create_memo(move |_| {
        action.value().with(|f| {
            f.clone().map(|g|
                g.map(|r1|
                    r1.unwrap_or_default()
                ).unwrap_or_default()
            ).unwrap_or_default()
        })
    });

    let id = create_memo(move |_| result.with(|bif| bif.id.to_string()));

    let name = create_memo(move |_| result.with(|bif| bif.name.clone()));
    let name_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("name").cloned()
        )
    });

    let desc = create_memo(move |_| result.with(|bif| bif.description.clone()));
    let desc_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("description").cloned()
        )
    });

    let quantity = create_memo(move |_| {
        result.with(|bif| bif.quantity.to_string())
    });
    let quantity_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("quantity").cloned()
        )
    });

    let size = create_memo(move |_| result.with(|bif| bif.size.to_string()));
    let size_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("size").cloned()
        )
    });

    let infinite = create_memo(move |_| result.with(|bif| bif.infinite));
    let infinite_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("infinite").cloned()
        )
    });

    let other_error = Signal::derive(move || {
        submit_error.with(|em|
            em.get("other").cloned()
        )
    });

    let on_submit = move |ev:SubmitEvent| {
        let data = CreateUpdateBagItem::from_event(&ev)
            .map(|it| it.item.validate());
        logging::log!("Data is {:?}", data);
        ev.prevent_default();

        match data {
            Ok(Some(RoadieAppError::MultipleErrors(e))) => {
                e.iter().for_each(|(key, value)| {
                    set_submit_error.update(|em| {em.insert(key.clone(), value.clone());});
                });
            }
            Err(e)  => {
                set_submit_error.update(|em| {em.insert("other".to_string(), e.to_string());});
            },
            Ok(Some(e)) => {
                set_submit_error.update(|em| {em.insert("other".to_string(), e.to_string());});
            }
            _ => ()
        };
    };

    view! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="bg-base-100 rounded-xl">
                    <div class="py-24 px-10 w-full">
                        <ActionForm action=action on:submit=on_submit >
                            <h2 class="text-2xl font-semibold mb-2 text-center">{move || submit_text()}</h2>
                            <InputText
                                field_label="Id"
                                container_style_base="invisible"
                                input_type="hidden"
                                field_name="item[id]"
                                field_value=id />
                            <InputText
                                field_label="Name"
                                field_value=name
                                placeholder = "Item Name"
                                field_name="item[name]"
                            />
                            <Alert alert_type="Error".into() msg=name_error />

                            <TextArea
                                field_label="Item Description"
                                field_value=desc
                                placeholder="Item Description"
                                field_name="item[description]" />
                            <Alert alert_type="Error".into() msg=desc_error />

                            <InputText
                                input_type="number"
                                field_label="Quantity"
                                field_value=quantity

                                field_name="item[quantity]"
                            />
                            <Alert alert_type="Error".into() msg=quantity_error />

                            <SelectBox
                                field_label="Item Size"
                                field_value=size
                                field_name="item[size]"
                                options=size_options />
                            <Alert alert_type="Error".into() msg=size_error />

                            <Checkbox
                                field_label="Is the supply infinite?"
                                field_value=infinite
                                field_name="item[infinite]" />
                            <Alert alert_type="Error".into() msg=infinite_error />
                            <button type="submit" class="btn mt-2 w-full btn-primary">{move || submit_text()}</button>
                            <Alert alert_type="Error".into() msg=other_error />
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}