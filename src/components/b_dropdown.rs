use leptos::html::Div;
use leptos::*;

use crate::elements::BButton;

#[component]
pub fn BDropdown<F, IV>(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] is_right: MaybeSignal<bool>,
    #[prop(optional, into)] is_up: MaybeSignal<bool>,
    #[prop(optional, into)] is_hoverable: MaybeSignal<bool>,
    trigger: F,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let node_ref = create_node_ref::<Div>();
    let is_active = create_rw_signal(false);

    let _ = leptos_use::on_click_outside(node_ref, move |_| {
        is_active.set(false);
    });

    let toggle_dropdown = move |_| {
        if is_hoverable.get() {
            is_active.set(false);
        } else {
            is_active.update(|value| *value = !*value);
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=format!("dropdown {}", class.get())
            class:is-active=is_active
            class:is-hoverable=is_hoverable
            class:is-right=is_right
            class:is-up=is_up
        >
            <div class="dropdown-trigger">
                <BButton on:click=toggle_dropdown>{trigger()}</BButton>
            </div>

            <div class="dropdown-menu">
                <div class="dropdown-content">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn BDropdownDivider() -> impl IntoView {
    view! { <hr class="dropdown-divider"/> }
}

#[component]
pub fn BDropdownItem(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional)] href: Option<&'static str>,
) -> impl IntoView
where
{
    view! {
        <a class=format!("dropdown-item {}", class.get()) href=href>
            {children()}
        </a>
    }
}
