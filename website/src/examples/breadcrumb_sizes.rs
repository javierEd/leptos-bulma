use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
use leptos_bulma::enums::BSize;

const BREADCRUMB_SIZE_OPTIONS: [BSize; 4] =
    [BSize::Small, BSize::Default, BSize::Medium, BSize::Large];

#[component]
pub fn BreadcrumbSizes() -> impl IntoView {
    view! {
        <For each=move || BREADCRUMB_SIZE_OPTIONS key=|bsize| *bsize let:bsize>
            <BBreadcrumb size=bsize>
                <BBreadcrumbItem href="#">"Has"</BBreadcrumbItem>
                <BBreadcrumbItem href="#">"size"</BBreadcrumbItem>
                <BBreadcrumbItem href="#" is_active=true>
                    {String::from(bsize)}
                </BBreadcrumbItem>
            </BBreadcrumb>
        </For>
    }
}
