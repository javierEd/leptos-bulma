use leptos::*;
use leptos_bulma::elements::{BBlock, BIcon, BIconText};
use leptos_bulma::icons::icondata_fa;

#[component]
pub fn BasicIcon() -> impl IntoView {
    view! {
        <BBlock>
            <BIcon is_large=true icon=icondata_fa::FaCss3AltBrands/>
            <BIcon is_medium=true icon=icondata_fa::FaCss3AltBrands/>
            <BIcon icon=icondata_fa::FaCss3AltBrands/>
            <BIcon is_small=true icon=icondata_fa::FaCss3AltBrands/>
            <BIconText icon=icondata_fa::FaCss3AltBrands text=move || "CSS3"/>
        </BBlock>
    }
}
