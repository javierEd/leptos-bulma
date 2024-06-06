use leptos::*;
use leptos_bulma::elements::{BTag, BTags};
use leptos_bulma::enums::{BColor, BSize};

const TAG_SIZE_OPTIONS: [BSize; 4] = [BSize::Normal, BSize::Default, BSize::Medium, BSize::Large];

#[component]
pub fn TagSizes() -> impl IntoView {
    view! {
        <BTags>
            <For each=move || TAG_SIZE_OPTIONS key=|bsize| *bsize let:bsize>
                <BTag color=BColor::Info size=bsize>
                    {String::from(bsize)}
                </BTag>
            </For>
        </BTags>
    }
}
