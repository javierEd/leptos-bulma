use leptos::*;
use leptos_bulma::elements::{BButton, BButtons};
use leptos_bulma::enums::BColor;

#[component]
pub fn BasicButtons() -> impl IntoView {
    view! {
        <BButtons>
            <BButton>"Normal button"</BButton>
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
