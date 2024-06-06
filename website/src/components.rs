use leptos::html::Code;
use leptos::*;
use leptos_bulma::elements::BBlock;
use leptos_meta::Title;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;

use crate::i18n::{t, use_i18n};

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | Leptos Bulma", text.get())/> }
}

#[wasm_bindgen(module = "/js/highlight.js")]
extern "C" {
    #[wasm_bindgen(js_name = highlightElement)]
    fn highlight_element(element: &HtmlElement);
}

#[component]
pub fn CodeBlock(
    children: Children,
    #[prop(default = "plaintext")] language: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Code>();

    create_effect(move |_| {
        if let Some(element) = node_ref.get() {
            if let Some(el) = element.dyn_ref::<HtmlElement>() {
                highlight_element(el)
            }
        }
    });

    view! {
        <pre class="block">
            <code node_ref=node_ref class=format!("language-{language}")>
                {children()}
            </code>
        </pre>
    }
}

#[component]
pub fn GoToBulmaIo(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <BBlock>
            {t!(i18n, additionally_you_can_check_bulma_official_documentation)} ": "
            <a href=format!("https://bulma.io/documentation/{path}") target="_blank">
                {format!("bulma.io/documentation/{path}")}
            </a>
        </BBlock>
    }
}

#[component]
pub fn GoToDocsRs(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();
    let version = env!("CARGO_PKG_VERSION");

    view! {
        <BBlock>
            {t!(i18n, to_find_more_information_you_can_go_to)} ": "
            <a href=format!("https://docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html") target="_blank">
                {format!("docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html")}
            </a>
        </BBlock>
    }
}
