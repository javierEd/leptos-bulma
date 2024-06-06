use leptos::*;
use leptos_bulma::elements::{BTag, BTags};
use leptos_bulma::enums::BColor;

fn tag_color_options() -> [BColor; 9] {
    [
        BColor::Custom("black".to_owned()),
        BColor::Custom("white".to_owned()),
        BColor::Danger,
        BColor::Default,
        BColor::Info,
        BColor::Link,
        BColor::Primary,
        BColor::Success,
        BColor::Warning,
    ]
}

#[component]
pub fn TagColors() -> impl IntoView {
    view! {
        <BTags>
            <For each=tag_color_options key=|bcolor| bcolor.clone() let:bcolor>
                <BTag color=bcolor.clone()>{String::from(bcolor)}</BTag>
            </For>
        </BTags>
    }
}
