use leptos::*;

#[component]
pub fn BButton(
    #[prop(optional, into)] button_type: Option<AttributeValue>,
    children: Children,
    #[prop(default = "")] class: &'static str,
    #[prop(optional, into)] is_loading: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <button
            class=format!("button {}", class)
            class:is-loading=move || is_loading.map(|v| v.get()).unwrap_or(false)
            type=button_type
        >
            {children()}
        </button>
    }
}
