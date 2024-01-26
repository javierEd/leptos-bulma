use leptos::html::Div;
use leptos::*;

use crate::MouseEventFn;

#[component]
pub fn BDropdown<F, IV>(
    children: Children,
    #[prop(default = "")] class: &'static str,
    trigger: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let node_ref = create_node_ref::<Div>();
    let (is_active, set_is_active) = create_signal(false);

    let _ = leptos_use::on_click_outside(node_ref, move |_| {
        set_is_active.set(false);
    });

    view! {
        <div node_ref=node_ref class=format!("dropdown {}", class) class:is-active=is_active>
            <div class="dropdown-trigger">
                <button class="button" on:click=move |_| { set_is_active.update(|v| *v = !*v) }>
                    {trigger()}
                </button>
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
    #[prop(default = "")] class: &'static str,
    #[prop(optional)] href: Option<&'static str>,
    #[prop(optional, into)] on_click: Option<MouseEventFn>,
) -> impl IntoView
where
{
    view! {
        <a class=format!("dropdown-item {}", class) href=href>
            {children()}
        </a>
    }
    .optional_event(ev::click, on_click.map(MouseEventFn::into_inner))
}
