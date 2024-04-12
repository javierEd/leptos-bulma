use leptos::*;
use leptos_bulma::elements::{BSubtitle, BTitle};

#[component]
pub fn BasicTitle() -> impl IntoView {
    view! {
        <BTitle is=1>"This is the title"</BTitle>
        <BSubtitle is=3>"This is the subtitle"</BSubtitle>
    }
}
