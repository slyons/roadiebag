use leptos::*;
use leptos_router::*;
use crate::auth::frontend::{AuthContext};
use crate::bag::api::*;
use crate::bag::model::TakenBagItem;
use leptos_use::{is_some, is_none};
#[component]
pub fn CurrentItem() -> impl IntoView {
    let (tbi, set_tbi) = create_signal(None);
    let taken_item = create_resource(
        || (),
        move |_| async move {
            if let Ok(Ok(Some(tbi))) = last_taken().await {
                logging::log!("Current item is {:?}", tbi);
                set_tbi(Some(tbi));
            }
        }
    );

    let take_item = create_action(move |()| async move{
        let item = take_random().await.expect("server error");
        taken_item.refetch();
        //taken_item.set(item);
    });

    let done_with_item = create_action(move |()| async move {
        let mut current_item = tbi().unwrap();
        current_item.done = true;
        update_taken(current_item).await;
        set_tbi(None);
    });

    let has_current_item = Signal::derive(move || {
        tbi.with(|t| t.is_some())
    });

    view! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="bg-base-100 rounded-xl">
                    <div class="py-24 px-10 w-full">
                        <div class="grid grid-cols gap-8 grid-cols-2 mt-10 sm:grid-cols-8 lg:grid-cols-12">
                            <Transition fallback=move || view! {} >
                                <div class="relative flex flex-col items-center col-span-12">
                                    <Show when=move || !has_current_item()>
                                        <button class="btn btn-primary" on:click=move |_| take_item.dispatch(())>Take Random</button>
                                    </Show>
                                    <Show when=has_current_item>
                                        <button class="btn btn-primary" on:click=move |_| done_with_item.dispatch(())>Done</button>
                                    </Show>
                                </div>
                                <Show when=has_current_item>
                                    <ItemDisplay item=tbi />
                                </Show>
                            </Transition>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ItemDisplay(
    #[prop(into)]
   item: Signal<Option<TakenBagItem>>) -> impl IntoView {
    view! {
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <div class="p-2 bg-neutral rounded-box text-neutral-content">
                <span class="font-mono text-5xl">
                    {move || item.get().map(|tbi| tbi.rounds).unwrap()}
                </span>
            </div>
            <div>
                <span class="text-sm">"rounds"</span>
            </div>
        </div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <h1>{move || item.get().map(|tbi| tbi.item.name)}</h1>
            <h3>{move || item.get().map(|tbi| tbi.item.size.to_string())}</h3>

        </div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">

        </div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <h4>{move || item.get().map(|tbi| tbi.item.description)}</h4>
        </div>
    }
}