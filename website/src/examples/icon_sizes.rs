use leptos::*;
use leptos_bulma::elements::{BBlock, BIcon, BIconText};
use leptos_bulma::enums::BSize;
use leptos_bulma::icons::icondata_fa;

const ICON_SIZE_OPTIONS: [BSize; 4] = [BSize::Small, BSize::Default, BSize::Medium, BSize::Large];

#[component]
pub fn IconSizes() -> impl IntoView {
    view! {
        <For each=move || ICON_SIZE_OPTIONS key=|bsize| *bsize let:bsize>
            <BBlock>
                <BIcon class="has-background-info-light" size=bsize icon=icondata_fa::FaCss3AltBrands/>
                <BIconText
                    icon_class="has-background-info-light ml-6"
                    size=bsize
                    icon=icondata_fa::FaCss3AltBrands
                    text=move || String::from(bsize)
                />
                <BIconText
                    icon_class="has-background-info-light ml-6"
                    size=bsize
                    is_scaled=true
                    icon=icondata_fa::FaCss3AltBrands
                    text=move || format!("{} scaled", String::from(bsize))
                />
            </BBlock>
        </For>
    }
}
