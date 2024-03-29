use leptos::*;

use crate::components::PageTitle;
use crate::i18n::{t, use_i18n};

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, home)/>

        <div class="has-text-centered">
            <h2 class="title is-3">
                <b>"Leptos Bulma"</b>" is a "<a href="https://leptos.dev" target="_blank">"Leptos"</a>
                " component library based on "<a href="https://bulma.io" target="_blank">"Bulma"</a>" CSS framework."
            </h2>
            <h3 class="subtitle">"This website is meant to be a guide on how to use it on your Leptos application."</h3>

            <p class="block">
                <a class="mx-1" href="https://crates.io/crates/leptos-bulma" target="_blank">
                    <img src="https://img.shields.io/crates/v/leptos-bulma.svg" alt="crates.io"/>
                </a>
                <a class="mx-1" href="https://docs.rs/leptos-bulma" target="_blank">
                    <img src="https://docs.rs/leptos-bulma/badge.svg" alt="docs.rs"/>
                </a>
            </p>
        </div>
    }
}
