use leptos::*;

#[component]
pub fn FormField(
    #[prop(into)] field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)] label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)] container_style: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format!("{} {}", container_style_base, container_style)>
            <label class="label">
                <span class=move || {
                    format!("{} {}", label_style_base, label_style)
                }>{field_label}</span>
            </label>
            {children()}
        </div>
    }
}

#[component]
pub fn InputText(
    #[prop(into)] field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)] label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)] container_style: String,

    #[prop(optional, default="input input-bordered w-full leading-6".to_string(), into)]
    field_style_base: String,
    #[prop(optional, default="".to_string(), into)] field_style: String,

    #[prop(optional, default="".to_string(), into)] placeholder: String,
    #[prop(optional, default="text".to_string(), into)] input_type: String,
    #[prop(into)] field_name: String,
    #[prop(optional, into)] field_value: Signal<String>,
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style
        >
            <input
                type=input_type
                name=field_name
                prop:value=move || { field_value() }
                placeholder=placeholder
                class=move || format!("{} {}", field_style_base, field_style)
            />

        </FormField>
    }
}

#[component]
pub fn TextArea(
    #[prop(into)] field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)] label_style: String,
    #[prop(optional, default="form-control w-full leading-6".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)] container_style: String,

    #[prop(optional, default="input input-bordered w-full py-3".to_string(), into)]
    field_style_base: String,
    #[prop(optional, default="".to_string(), into)] field_style: String,

    #[prop(optional, default="".to_string(), into)] placeholder: String,
    #[prop(into)] field_name: String,
    #[prop(into)] field_value: Signal<String>,
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style
        >
            <textarea
                name=field_name
                prop:value=move || { field_value() }
                placeholder=placeholder
                class=move || format!("{} {}", field_style_base, field_style)
            ></textarea>

        </FormField>
    }
}

#[component]
pub fn Checkbox(
    #[prop(into)] field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)] label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)] container_style: String,

    #[prop(optional, default="toggle toggle-lg".to_string(), into)] field_style_base: String,
    #[prop(optional, default="".to_string(), into)] field_style: String,

    #[prop(into)] field_name: String,
    #[prop(into)] field_value: Signal<bool>,
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style
        >
            <input
                type="checkbox"
                name=field_name
                prop:value=field_value
                class=move || format!("{} {}", field_style_base, field_style)
                checked=move || field_value()
            />

        </FormField>
    }
}

#[component]
pub fn SelectBox(
    #[prop(into)] field_label: String,
    #[prop(optional, default="label-text text-base-content ".to_string(), into)]
    label_style_base: String,
    #[prop(optional, default="".to_string(), into)] label_style: String,
    #[prop(optional, default="form-control w-full ".to_string(), into)]
    container_style_base: String,
    #[prop(optional, default="".to_string(), into)] container_style: String,

    #[prop(optional, default="select select-bordered w-full max-w-xs".to_string(), into)]
    field_style_base: String,
    #[prop(optional, default="".to_string(), into)] field_style: String,

    #[prop(into)] field_name: String,
    #[prop(into)] field_value: Signal<String>,
    #[prop(into)] options: MaybeSignal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        <FormField
            field_label=field_label
            label_style_base=label_style_base
            label_style=label_style
            container_style_base=container_style_base
            container_style=container_style
        >
            <select name=field_name class=move || format!("{} {}", field_style_base, field_style)>
                <option disabled attr:selected=move || field_value.with(|c| c == "Unknown")>
                    Pick one
                </option>
                <For
                    each=options
                    key=|option| option.0.clone()
                    children=move |(id, txt)| {
                        view! {
                            <option attr:select=move || {
                                field_value.with(|c| c == &id)
                            }>{txt}</option>
                        }
                    }
                />

            </select>

        </FormField>
    }
}
