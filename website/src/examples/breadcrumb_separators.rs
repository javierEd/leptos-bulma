use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
use leptos_bulma::enums::BBreadcrumbSeparator;

const BREADCRUMB_SEPARATOR_OPTIONS: [BBreadcrumbSeparator; 5] = [
    BBreadcrumbSeparator::Arrow,
    BBreadcrumbSeparator::Bullet,
    BBreadcrumbSeparator::Default,
    BBreadcrumbSeparator::Dot,
    BBreadcrumbSeparator::Succeeds,
];

#[component]
pub fn BreadcrumbSeparators() -> impl IntoView {
    view! {
        <For each=move || BREADCRUMB_SEPARATOR_OPTIONS key=|bseparator| *bseparator let:bseparator>
            <BBreadcrumb separator=bseparator>
                <BBreadcrumbItem href="#">"Is"</BBreadcrumbItem>
                <BBreadcrumbItem href="#">"separated"</BBreadcrumbItem>
                <BBreadcrumbItem href="#">"by"</BBreadcrumbItem>
                <BBreadcrumbItem href="#" is_active=true>
                    {String::from(bseparator)}
                </BBreadcrumbItem>
            </BBreadcrumb>
        </For>
    }
}
