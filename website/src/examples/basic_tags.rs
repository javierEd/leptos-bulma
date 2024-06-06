use leptos::*;
use leptos_bulma::elements::{BTag, BTags};

#[component]
pub fn BasicTags() -> impl IntoView {
    view! {
        <BTags>
            <BTag>"Normal tag"</BTag>
            <BTag is_light=true>"Light tag"</BTag>
            <BTag is_dark=true>"Dark tag"</BTag>
            <BTag is_hoverable=true>"Hoverable tag"</BTag>
            <BTag is_rounded=true>"Rounded tag"</BTag>
        </BTags>
    }
}
