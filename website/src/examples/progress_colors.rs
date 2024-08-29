use leptos::*;
use leptos_bulma::elements::BProgress;
use leptos_bulma::enums::BColor;

const PROGRESS_COLOR_OPTIONS: [BColor; 7] = [
    BColor::Danger,
    BColor::Default,
    BColor::Info,
    BColor::Link,
    BColor::Primary,
    BColor::Success,
    BColor::Warning,
];

#[component]
pub fn ProgressColors() -> impl IntoView {
    view! {
        <For each=move || PROGRESS_COLOR_OPTIONS key=|bcolor| bcolor.clone() let:bcolor>
            <BProgress color=bcolor.clone() max=100 value=25 />
        </For>
    }
}
