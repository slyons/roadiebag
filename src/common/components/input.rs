use leptos::*;

#[component]
pub fn FormField(
    #[prop(into)]
    field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,
    children: Children
) -> impl IntoView {
    view! {
        <div class=move|| format!("{} {}", container_style_base, container_style)>
            <label class="label">
                <span class= move || format!("{} {}", label_style_base, label_style)>{field_label}</span>
            </label>
            {children()}
        </div>
    }
}

#[component]
pub fn InputText(
    #[prop(optional, into)]
    label_title: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="text".to_string(), into)]
    input_type: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,
    #[prop(optional, default="".to_string(), into)]
    default_value: String,
    #[prop(optional, default="".to_string(), into)]
    placeholder: String,
    #[prop(into)]
    name: String
) -> impl IntoView {
    view! {
        <div class=move|| format!("form-control w-full {}", container_style)>
            <label class="label">
                <span class= move || format!("label-text text-base-content {}", label_style)>{label_title}</span>
            </label>
            <input type=input_type name=name prop:value=default_value placeholder=placeholder class="input input-bordered w-full" />
        </div>
    }
}

#[component]
pub fn InputControlled(
    #[prop(into)]
    field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,

    #[prop(optional, default="input input-bordered w-full ".to_string(), into)]
    input_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    input_style: String,

    #[prop(optional, default="text".to_string(), into)]
    input_type: String,
    #[prop(optional, default="".to_string(), into)]
    placeholder: String,
    #[prop(into)]
    field_name: String,
    #[prop(into)]
    value: Signal<String>,
    #[prop(into)]
    set_value: SignalSetter<String>
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style>
            <input type=input_type name=field_name prop:value=value
                placeholder=placeholder class=move || format!("{} {}", input_style_base, input_style)
                on:input=move |ev| set_value(event_target_value(&ev))
            />
        </FormField>
    }
}

#[component]
pub fn TextareaControlled(
    #[prop(into)]
    field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,

    #[prop(optional, default="textarea textarea-bordered h-24 ".to_string(), into)]
    input_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    input_style: String,

    #[prop(optional, default="text".to_string(), into)]
    input_type: String,
    #[prop(optional, default="".to_string(), into)]
    placeholder: String,
    #[prop(into)]
    field_name: String,
    #[prop(into)]
    value: Signal<String>,
    #[prop(into)]
    set_value: SignalSetter<String>
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style>
            <textarea name=field_name prop:value=value
                placeholder=placeholder class=move || format!("{} {}", input_style_base, input_style)
                on:input=move |ev| set_value(event_target_value(&ev))
            />
        </FormField>
    }
}

#[component]
pub fn CheckboxControlled(
    #[prop(into)]
    field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,
    #[prop(optional, default="toggle ".to_string(), into)]
    input_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    input_style: String,

    #[prop(into)]
    field_name: String,
    #[prop(into)]
    value: Signal<bool>,
    #[prop(into)]
    set_value: SignalSetter<bool>
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style>
            <input type="checkbox" name=field_name prop:value=value
                class=move || format!("{} {}", input_style_base, input_style)
                checked=value
                on:input=move |ev| set_value(event_target_checked(&ev))
            />
        </FormField>
    }
}



#[component]
pub fn SelectControlled(
    #[prop(into)]
    field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    container_style: String,
    #[prop(optional, default="select select-bordered ".to_string(), into)]
    input_style_base: String,
    #[prop(optional, default="".to_string(), into)]
    input_style: String,

    #[prop(into)]
    field_name: String,
    #[prop(into)]
    options: MaybeSignal<Vec<(String, String)>>,
    #[prop(into)]
    selected: Signal<Option<String>>,
    #[prop(into)]
    set_selected: SignalSetter<Option<String>>
) -> impl IntoView {
    let current_value = Signal::derive(move || {
        selected.with(|s| s.clone().unwrap_or("_DEFAULT".into()))
    });
    view! {
        <div class=move|| format!("form-control w-full max-w-xs {}", container_style)>
            <label class="label">
                <span class= move || format!("label-text text-base-content {}", label_style)>{field_label}</span>
            </label>
            <select name=field_name class=move || format!("{} {}", input_style_base, input_style)
                on:change = move |ev| {
                    let value = event_target_value(&ev);
                    logging::log!("Setting select value to {}", value);
                    set_selected(Some(value))
                 }
            >
                <option disabled attr:selected=move || current_value.with(|c| c == "_DEFAULT")>Pick one</option>
                <For
                    each=options
                    key=|option| option.0.clone()
                    children=move |(id, txt)| {
                        view! {
                            <option attr:select=move || current_value.with(|c| c == &id)>{txt}</option>
                        }
                    }
                />
            </select>
        </div>
    }
}

