use crate::bag::api::*;
use crate::bag::model::TakenBagItem;

use leptos::*;
use crate::errors::*;

#[component]
pub fn CurrentItem() -> impl IntoView {
    let (tbi, set_tbi) = create_signal(Ok(None));
    let taken_item = create_resource(
        || (),
        move |_| async move {
            let to_set = last_taken().await.into_rr();

            /*let to_set = match last_taken().await {
                Err(e) => Err(RoadieAppError::ServerError(e)),
                Ok(Err(e)) => Err(e),
                Ok(Ok(stbi)) => Ok(stbi)
            };*/
            logging::log!("Current item is {:?}", to_set);
            set_tbi(to_set);
        },
    );

    let take_item = create_action(move |()| async move {
        let _item = take_random().await.expect("server error");
        taken_item.refetch();
    });

    let done_with_item = create_action(move |()| async move {
        if let Ok(Some(mut current_item)) = tbi() {
            current_item.done = true;
            let to_set = update_taken(current_item).await.into_rr();
            let to_set = to_set.map(|_v| None);
            set_tbi(to_set);
        }
    });

    let has_current_item = Signal::derive(move || {
        matches!(tbi(), Ok(Some(_)))
    });

    view! {
        <div class="min-h-screen bg-base-200 flex items-center">
            <div class="card mx-auto w-full max-w-5xl  shadow-xl">
                <div class="bg-base-100 rounded-xl">
                    <div class="py-24 px-10 w-full">
                        <div class="grid grid-cols gap-8 grid-cols-2 mt-10 sm:grid-cols-8 lg:grid-cols-12">
                            <Transition fallback=move || view! {}>
                                <div class="relative flex flex-col items-center col-span-12">
                                    <Show when=move || !has_current_item()>
                                        <button
                                            class="btn btn-primary"
                                            on:click=move |_| take_item.dispatch(())
                                        >
                                            Take Random
                                        </button>
                                    </Show>
                                    <Show when=has_current_item>
                                        <button
                                            class="btn btn-primary"
                                            on:click=move |_| done_with_item.dispatch(())
                                        >
                                            Done
                                        </button>
                                    </Show>
                                </div>
                                <Show when=has_current_item>
                                    <ItemDisplay item=tbi/>
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
pub fn ItemDisplay(#[prop(into)] item: Signal<RoadieResult<Option<TakenBagItem>>>) -> impl IntoView {

    view! {
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <div class="p-2 bg-neutral rounded-box text-neutral-content">
                <span class="font-mono text-5xl">
                    {move || item.get().map(|tbi| tbi.unwrap().rounds)}
                </span>
            </div>
            <div>
                <span class="text-sm">"rounds"</span>
            </div>
        </div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <h1>{move || item.get().map(|tbi| tbi.unwrap().item.name)}</h1>
            <h3>{move || item.get().map(|tbi| tbi.unwrap().item.size.to_string())}</h3>

        </div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl"></div>
        <div class="relative flex flex-col items-center justify-between col-span-6 px-8 py-12 space-y-4 overflow-hidden sm:rounded-xl">
            <h4>{move || item.get().map(|tbi| tbi.unwrap().item.description)}</h4>
        </div>
    }
}
