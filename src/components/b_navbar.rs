use leptos::ev;
use leptos::html::{Div, A};
use leptos::*;

use crate::MouseEventFn;

#[component]
pub fn BNavbar(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <nav class=format!("navbar {}", class)>{children()}</nav> }
}

#[component]
pub fn BNavbarBrand(children: Children) -> impl IntoView {
    view! { <nav class="navbar-brand">{children()}</nav> }
}

#[component]
pub fn BNavbarBurger(is_active: RwSignal<bool>) -> impl IntoView {
    let burger_ref = create_node_ref::<A>();

    let _ = leptos_use::on_click_outside(burger_ref, move |_| {
        is_active.set(false);
    });

    view! {
        <a
            node_ref=burger_ref
            class="navbar-burger"
            class:is-active=is_active
            on:click=move |_| { is_active.update(|v| *v = !*v) }
        >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    }
}

#[component]
pub fn BNavbarDivider() -> impl IntoView {
    view! { <hr class="navbar-divider"/> }
}

#[component]
pub fn BNavbarEnd(children: Children) -> impl IntoView {
    view! { <div class="navbar-end">{children()}</div> }
}

#[component]
pub fn BNavbarItem(
    children: Children,
    #[prop(default = "")] class: &'static str,
    #[prop(optional)] href: Option<&'static str>,
    #[prop(optional, into)] on_click: Option<MouseEventFn>,
) -> impl IntoView
where
{
    view! {
        <a class=format!("navbar-item {}", class) href=href>
            {children()}
        </a>
    }
    .optional_event(ev::click, on_click.map(MouseEventFn::into_inner))
}

#[component]
pub fn BNavbarItemDropdown<F, IV>(
    children: Children,
    #[prop(default = "")] dropdown_class: &'static str,
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
        <div
            node_ref=node_ref
            class="navbar-item has-dropdown"
            class:is-active=is_active
            on:click=move |_| { set_is_active.update(|v| *v = !*v) }
        >
            <a class="navbar-link">{trigger()}</a>

            <div class=format!("navbar-dropdown {}", dropdown_class)>{children()}</div>
        </div>
    }
}

#[component]
pub fn BNavbarMenu(
    children: Children,
    #[prop(optional, into)] is_active: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <div class="navbar-menu" class:is-active=move || is_active.map(|v| v.get()) == Some(true)>
            {children()}
        </div>
    }
}

#[component]
pub fn BNavbarStart(children: Children) -> impl IntoView {
    view! { <div class="navbar-start">{children()}</div> }
}
