use leptos::*;

#[cfg(not(feature = "leptos-icons"))]
#[component]
pub fn BIcon(
    #[prop(optional, into)] is_large: MaybeSignal<bool>,
    #[prop(optional, into)] is_medium: MaybeSignal<bool>,
    #[prop(optional, into)] is_small: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] is_scaled: MaybeSignal<bool>,
    #[prop(optional, into)] class: TextProp,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class=format!("icon {}", class.get())
            class:is-large=is_large
            class:is-medium=is_medium
            class:is-small=is_small
            class:is-scaled=is_scaled
        >
            {children()}
        </span>
    }
}

#[cfg(feature = "leptos-icons")]
#[component]
pub fn BIcon(
    #[prop(into)] icon: MaybeSignal<icondata_core::Icon>,
    #[prop(optional, into)] is_large: MaybeSignal<bool>,
    #[prop(optional, into)] is_medium: MaybeSignal<bool>,
    #[prop(optional, into)] is_small: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] is_scaled: MaybeSignal<bool>,
    #[prop(optional, into)] class: TextProp,
) -> impl IntoView {
    use leptos_icons::Icon;

    view! {
        <span
            class=format!("icon {}", class.get())
            class:is-large=is_large
            class:is-medium=is_medium
            class:is-small=is_small
            class:is-scaled=is_scaled
        >
            <Icon icon=icon/>
        </span>
    }
}

#[cfg(feature = "leptos-icons")]
#[component]
pub fn BIconText<F, IV>(
    #[prop(into)] icon: MaybeSignal<icondata_core::Icon>,
    #[prop(optional, into)] is_large: MaybeSignal<bool>,
    #[prop(optional, into)] is_medium: MaybeSignal<bool>,
    #[prop(optional, into)] is_small: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] is_scaled: MaybeSignal<bool>,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] icon_class: TextProp,
    text: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <span class=format!("icon-text {}", class.get())>
            <BIcon
                is_large=is_large
                is_medium=is_medium
                is_small=is_small
                is_scaled=is_scaled
                icon=icon
                class=icon_class
            />
            <span>{text()}</span>
        </span>
    }
}

#[cfg(not(feature = "leptos-icons"))]
#[component]
pub fn BIconText<F, IV>(
    #[prop(optional, into)] is_large: MaybeSignal<bool>,
    #[prop(optional, into)] is_medium: MaybeSignal<bool>,
    #[prop(optional, into)] is_small: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] is_scaled: MaybeSignal<bool>,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] icon_class: TextProp,
    text: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <span class=format!("icon-text {}", class.get())>
            <BIcon is_large=is_large is_medium=is_medium is_small=is_small is_scaled=is_scaled class=icon_class>
                {children()}
            </BIcon>
            <span>{text()}</span>
        </span>
    }
}
