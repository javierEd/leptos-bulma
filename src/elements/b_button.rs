use leptos::*;

use crate::enums::{BColor, BSize, BState};

fn get_button_class_list(
    class: Oco<'static, str>,
    color: BColor,
    size: BSize,
    state: BState,
    is_dark: bool,
    is_fullwidth: bool,
    is_inverted: bool,
    is_light: bool,
    is_outlined: bool,
    is_responsive: bool,
    is_rounded: bool,
) -> String {
    let mut class_list = "button".to_owned();

    if color != BColor::Default {
        class_list += &format!(" is-{}", String::from(color))
    };

    if size != BSize::Default {
        class_list += &format!(" is-{}", String::from(size))
    };

    if ![BState::Default, BState::Disabled].contains(&state) {
        class_list += &format!(" is-{}", String::from(state))
    };

    if is_dark {
        class_list += " is-dark";
    }

    if is_fullwidth {
        class_list += " is-fullwidth";
    }

    if is_inverted {
        class_list += " is-inverted";
    }

    if is_light {
        class_list += " is-light";
    }

    if is_outlined {
        class_list += " is-outlined";
    }

    if is_responsive {
        class_list += " is-responsive";
    }

    if is_rounded {
        class_list += " is-rounded";
    }

    if !class.is_empty() {
        class_list += &format!(" {}", class);
    }

    class_list
}

#[component]
pub fn BAButton(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: MaybeSignal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(default = BState::Default.into(), into)] state: MaybeSignal<BState>,
    #[prop(optional, into)] is_dark: MaybeSignal<bool>,
    #[prop(optional, into)] is_fullwidth: MaybeSignal<bool>,
    #[prop(optional, into)] is_inverted: MaybeSignal<bool>,
    #[prop(optional, into)] is_light: MaybeSignal<bool>,
    #[prop(optional, into)] is_outlined: MaybeSignal<bool>,
    #[prop(optional, into)] is_responsive: MaybeSignal<bool>,
    #[prop(optional, into)] is_rounded: MaybeSignal<bool>,
    #[prop(optional, into)] href: Option<TextProp>,
    #[prop(optional, into)] target: Option<TextProp>,
    #[prop(optional, into)] title: Option<TextProp>,
) -> impl IntoView {
    let button_class_list = move || {
        get_button_class_list(
            class.get(),
            color.get(),
            size.get(),
            state.get(),
            is_dark.get(),
            is_fullwidth.get(),
            is_inverted.get(),
            is_light.get(),
            is_outlined.get(),
            is_responsive.get(),
            is_rounded.get(),
        )
    };

    view! {
        <a class=button_class_list href=href target=target title=title disabled=move || state.get() == BState::Disabled>
            {children()}
        </a>
    }
}

#[component]
pub fn BButton(
    #[prop(optional, into)] button_type: Option<AttributeValue>,
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: MaybeSignal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(default = BState::Default.into(), into)] state: MaybeSignal<BState>,
    #[prop(optional, into)] is_dark: MaybeSignal<bool>,
    #[prop(optional, into)] is_fullwidth: MaybeSignal<bool>,
    #[prop(optional, into)] is_inverted: MaybeSignal<bool>,
    #[prop(optional, into)] is_light: MaybeSignal<bool>,
    #[prop(optional, into)] is_outlined: MaybeSignal<bool>,
    #[prop(optional, into)] is_responsive: MaybeSignal<bool>,
    #[prop(optional, into)] is_rounded: MaybeSignal<bool>,
    #[prop(optional, into)] title: Option<TextProp>,
) -> impl IntoView {
    let button_class_list = move || {
        get_button_class_list(
            class.get(),
            color.get(),
            size.get(),
            state.get(),
            is_dark.get(),
            is_fullwidth.get(),
            is_inverted.get(),
            is_light.get(),
            is_outlined.get(),
            is_responsive.get(),
            is_rounded.get(),
        )
    };

    view! {
        <button class=button_class_list type=button_type title=title disabled=move || state.get() == BState::Disabled>
            {children()}
        </button>
    }
}

#[component]
pub fn BButtons(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(optional, into)] has_addons: MaybeSignal<bool>,
) -> impl IntoView {
    let buttons_class_list = move || {
        let mut class_list = "buttons".to_owned();

        if size.get() != BSize::Default {
            class_list += &format!(" are-{}", String::from(size.get()))
        };

        if has_addons.get() {
            class_list += " has-addons";
        }

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
    };

    view! { <div class=buttons_class_list>{children()}</div> }
}
