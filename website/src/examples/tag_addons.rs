use leptos::*;
use leptos_bulma::elements::{BTag, BTags};
use leptos_bulma::enums::BColor;

#[component]
pub fn TagAddons() -> impl IntoView {
    view! {
        <BTags has_addons=true>
            <BTag color=BColor::Danger>"With delete"</BTag>
            <BTag is_dark=true is_delete=true/>
        </BTags>
    }
}
