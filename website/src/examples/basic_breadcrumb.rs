use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};

#[component]
pub fn BasicBreadcrumb() -> impl IntoView {
    view! {
        <BBreadcrumb>
            <BBreadcrumbItem href="#">"Leptos Bulma"</BBreadcrumbItem>
            <BBreadcrumbItem href="#">"Components"</BBreadcrumbItem>
            <BBreadcrumbItem href="#" is_active=true>
                "Breadcrumb"
            </BBreadcrumbItem>
        </BBreadcrumb>
    }
}
