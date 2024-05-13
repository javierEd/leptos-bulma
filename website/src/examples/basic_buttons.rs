use leptos::*;
use leptos_bulma::elements::{BAButton, BButton, BButtons};
use leptos_bulma::enums::BColor;

#[component]
pub fn BasicButtons() -> impl IntoView {
    view! {
        <BButtons>
            <BButton>"Normal button"</BButton>
            <BAButton href="/elements#button" target="_blank" title="Open in a new tab">
                "Anchor link button"
            </BAButton>
            <BButton is_light=true>"Light button"</BButton>
            <BButton is_dark=true>"Dark button"</BButton>
            <BButton is_inverted=true>"Inverted button"</BButton>
            <BButton is_outlined=true color=BColor::Primary>
                "Outlined button"
            </BButton>
            <BButton is_rounded=true>"Rounded button"</BButton>
            <BButton is_fullwidth=true>"Fullwidth button"</BButton>
        </BButtons>
    }
}
