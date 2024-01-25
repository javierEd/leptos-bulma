use leptos::html::A;
use leptos::*;

#[component]
pub fn BNavbar(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <nav class=format!("navbar {}", class)>{children()}</nav> }
}

#[component]
pub fn BNavbarBrand(children: Children) -> impl IntoView {
    view! { <nav class="navbar-brand">{children()}</nav> }
}

#[component]
pub fn BNavbarBurger(
    is_active: ReadSignal<bool>,
    set_is_active: WriteSignal<bool>,
) -> impl IntoView {
    let burger_ref = create_node_ref::<A>();

    let _ = leptos_use::on_click_outside(burger_ref, move |_| {
        set_is_active.set(false);
    });

    view! {
        <a
            node_ref=burger_ref
            class="navbar-burger"
            class:is-active=is_active
            on:click=move |_| { set_is_active.update(|v| *v = !*v) }
        >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    }
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
) -> impl IntoView {
    view! {
        <a class=format!("navbar-item {}", class) href=href>
            {children()}
        </a>
    }
}

#[component]
pub fn BNavbarMenu(
    children: Children,
    #[prop(optional)] is_active: Option<ReadSignal<bool>>,
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
