use leptos::*;

use crate::enums::{BColor, BSize, BState};

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
    #[prop(optional, into)] title: TextProp,
) -> impl IntoView {
    let button_class_list = move || {
        let mut class_list = "button".to_owned();

        if color.get() != BColor::Default {
            class_list += &format!(" is-{}", String::from(color.get()))
        };

        if size.get() != BSize::Default {
            class_list += &format!(" is-{}", String::from(size.get()))
        };

        if ![BState::Default, BState::Disabled].contains(&state.get()) {
            class_list += &format!(" is-{}", String::from(state.get()))
        };

        if is_dark.get() {
            class_list += " is-dark";
        }

        if is_fullwidth.get() {
            class_list += " is-fullwidth";
        }

        if is_inverted.get() {
            class_list += " is-inverted";
        }

        if is_light.get() {
            class_list += " is-light";
        }

        if is_outlined.get() {
            class_list += " is-outlined";
        }

        if is_responsive.get() {
            class_list += " is-responsive";
        }

        if is_rounded.get() {
            class_list += " is-rounded";
        }

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
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
