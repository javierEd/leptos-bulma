use leptos::*;

#[component]
pub fn BNavbar(children: Children) -> impl IntoView {
    view! {
        <nav class="navbar">{children()}</nav>
    }
}

#[component]
pub fn BNavbarBrand(children: Children) -> impl IntoView {
    view! {
        <nav class="navbar-brand">{children()}</nav>
    }
}
