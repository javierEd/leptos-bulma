use leptos::*;

use crate::components::PageTitle;
use crate::i18n::{t, use_i18n};

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, home)/>

        <div>
            <h3 class="title is-4">
                <b>"Leptos Bulma"</b>" is a "<a href="https://leptos.dev" target="_blank">"Leptos"</a>
                " component library based on "<a href="https://bulma.io" target="_blank">"Bulma"</a>" CSS framework."
            </h3>
            <h4 class="subtitle">"This website is meant to be a guide on how to use it on your Leptos application."</h4>
        </div>
    }
}
