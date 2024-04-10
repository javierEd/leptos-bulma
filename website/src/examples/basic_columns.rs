use leptos::*;
use leptos_bulma::{
    columns::{BColumn, BColumns},
    elements::BBox,
};

#[component]
pub fn BasicColumns() -> impl IntoView {
    view! {
        <BColumns>
            <BColumn is="two-fifths">
                <BBox class="has-background-primary has-text-black">"First column"</BBox>
            </BColumn>
            <BColumn is="3">
                <BBox class="has-background-light has-text-black">"Second column"</BBox>
            </BColumn>
            <BColumn>
                <BBox class="has-background-info has-text-black">"Third column"</BBox>
            </BColumn>
            <BColumn is="narrow">
                <BBox class="has-background-success has-text-black">"Fourth column"</BBox>
            </BColumn>
        </BColumns>
    }
}
