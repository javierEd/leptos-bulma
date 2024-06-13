use leptos::*;

use crate::enums::BSize;

fn get_icon_class_list(class: Oco<'static, str>, size: BSize, is_scaled: bool) -> String {
    let mut class_list = "icon".to_owned();

    size.add_to_class_list(&mut class_list);

    if is_scaled {
        class_list += " is-scaled";
    }

    if !class.is_empty() {
        class_list += &format!(" {}", class);
    }

    class_list
}

#[cfg(feature = "leptos-icons")]
#[component]
pub fn BIcon(
    #[prop(optional, into)] class: TextProp,
    #[prop(into)] icon: MaybeSignal<icondata_core::Icon>,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(optional, into)] is_scaled: MaybeSignal<bool>,
) -> impl IntoView {
    use leptos_icons::Icon;

    let icon_class_list = move || get_icon_class_list(class.get(), size.get(), is_scaled.get());

    view! {
        <span class=icon_class_list>
            <Icon icon=icon/>
        </span>
    }
}

#[cfg(not(feature = "leptos-icons"))]
#[component]
pub fn BIcon(
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(optional, into)] is_scaled: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let icon_class_list = move || get_icon_class_list(class.get(), size.get(), is_scaled.get());

    view! { <span class=icon_class_list>{children()}</span> }
}

#[cfg(feature = "leptos-icons")]
#[component]
pub fn BIconText<F, IV>(
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] icon_class: TextProp,
    #[prop(optional, into)] text_class: TextProp,
    #[prop(into)] icon: MaybeSignal<icondata_core::Icon>,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(optional, into)] is_scaled: MaybeSignal<bool>,
    text: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <span class=move || format!("icon-text {}", class.get())>
            <BIcon class=icon_class size=size is_scaled=is_scaled icon=icon/>
            <span class=text_class>{text()}</span>
        </span>
    }
}

#[cfg(not(feature = "leptos-icons"))]
#[component]
pub fn BIconText<F, IV>(
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] icon_class: TextProp,
    #[prop(optional, into)] text_class: TextProp,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(optional, into)] is_scaled: MaybeSignal<bool>,
    text: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <span class=move || format!("icon-text {}", class.get())>
            <BIcon size=size is_scaled=is_scaled class=icon_class>
                {children()}
            </BIcon>
            <span class=text_class>{text()}</span>
        </span>
    }
}
