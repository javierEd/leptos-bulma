use leptos::*;
use leptos_bulma::elements::BProgress;

#[component]
pub fn ProgressIndeterminate() -> impl IntoView {
    view! { <BProgress max=100 /> }
}
