use leptos::*;
use leptos_bulma::elements::BBlock;

#[component]
pub fn BasicBlock() -> impl IntoView {
    view! { <BBlock class="has-text-centered">"Hello, World!"</BBlock> }
}
