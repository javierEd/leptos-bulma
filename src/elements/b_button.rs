use leptos::*;

#[component]
pub fn BButton(
    #[prop(optional, into)] button_type: Option<AttributeValue>,
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] is_active: MaybeSignal<bool>,
    #[prop(optional, into)] is_loading: MaybeSignal<bool>,
    #[prop(optional, into)] title: TextProp,
) -> impl IntoView {
    view! {
        <button
            class=format!("button {}", class.get())
            class:is-active=is_active
            class:is-loading=is_loading
            type=button_type
            title=title
        >
            {children()}
        </button>
    }
}

#[component]
pub fn BButtons(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] has_addons: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <div class=format!("buttons {}", class.get()) class:has-addons=has_addons>
            {children()}
        </div>
    }
}
