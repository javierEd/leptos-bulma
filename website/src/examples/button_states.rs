use leptos::*;
use leptos_bulma::elements::{BButton, BButtons};
use leptos_bulma::enums::BState;

fn button_state_options() -> [BState; 6] {
    [
        BState::Active,
        BState::Default,
        BState::Disabled,
        BState::Focused,
        BState::Hovered,
        BState::Loading,
    ]
}

#[component]
pub fn ButtonStates() -> impl IntoView {
    view! {
        <BButtons>
            <For each=button_state_options key=|bstate| *bstate let:bstate>
                <BButton state=bstate>{String::from(bstate)}</BButton>
            </For>
        </BButtons>
    }
}
