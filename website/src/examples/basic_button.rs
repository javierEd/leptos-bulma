use leptos::*;
use leptos_bulma::elements::{BButton, BButtons};

#[component]
pub fn BasicButton() -> impl IntoView {
    view! {
        <BButtons>
            <BButton>"Normal button"</BButton>
            <BButton is_active=true>"Active button"</BButton>
            <BButton is_loading=true>"Loading button"</BButton>
        </BButtons>
    }
}
