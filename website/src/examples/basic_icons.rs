use leptos::*;
use leptos_bulma::elements::{BIcon, BIconText};
use leptos_bulma::icons::icondata_fa;

#[component]
pub fn BasicIcons() -> impl IntoView {
    view! {
        <BIcon icon=icondata_fa::FaCss3AltBrands/>
        <BIconText class="ml-6" icon=icondata_fa::FaCss3AltBrands text=move || "with text"/>
        <BIconText class="ml-6" icon=icondata_fa::FaCss3AltBrands is_scaled=true text=move || "with text scaled"/>
    }
}
