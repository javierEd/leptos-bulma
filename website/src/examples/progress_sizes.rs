use leptos::*;
use leptos_bulma::elements::BProgress;
use leptos_bulma::enums::BSize;

const PROGRESS_SIZE_OPTIONS: [BSize; 4] =
    [BSize::Small, BSize::Default, BSize::Medium, BSize::Large];

#[component]
pub fn ProgressSizes() -> impl IntoView {
    view! {
        <For each=move || PROGRESS_SIZE_OPTIONS key=|bsize| *bsize let:bsize>
            <BProgress max=100 size=bsize value=25 />
        </For>
    }
}
