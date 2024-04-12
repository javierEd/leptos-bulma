use leptos::*;
use leptos_bulma::elements::{BSubtitle, BTitle};
use leptos_bulma::layout::BSection;

#[component]
pub fn BasicSection() -> impl IntoView {
    view! {
        <BSection class="has-text-centered">
            <BTitle>"This is a section"</BTitle>
            <BSubtitle>
                "A simple container to divide your page into " <strong>"sections"</strong>
                ", like the one you're currently reading."
            </BSubtitle>
        </BSection>
    }
}
