use leptos::*;
use leptos_meta::Title;

use crate::i18n::{t, use_i18n};

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | Leptos Bulma", text.get())/> }
}

#[component]
pub fn CodeBlock(children: Children) -> impl IntoView {
    view! {
        <pre class="block">
            <code>{children()}</code>
        </pre>
    }
}

#[component]
pub fn GoToBulmaIo(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="section">
            {t!(i18n, to_find_more_information_you_can_check_the_official_bulma_documentation)} ": "
            <a href=format!("https://bulma.io/documentation/{path}") target="_blank">
                {format!("bulma.io/documentation/{path}")}
            </a>
        </section>
    }
}

#[component]
pub fn GoToDocsRs(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();
    let version = env!("CARGO_PKG_VERSION");

    view! {
        <p class="block">
            {t!(i18n, to_find_more_information_you_can_go_to)} ": "
            <a href=format!("https://docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html") target="_blank">
                {format!("docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html")}
            </a>
        </p>
    }
}
