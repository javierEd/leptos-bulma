use leptos::*;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicBox, RustCodeExample};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ElementsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, elements)/>

        <h2 class="title is-3">{t!(i18n, elements)}</h2>

        <section class="section">
            <h3 class="title is-4">"Box"</h3>

            <p class="block">"Example:"</p>

            <RustCodeExample name="basic_box"/>

            <p class="block">"See it in action:"</p>

            <BasicBox/>

            <GoToDocsRs path="elements/fn.BBox"/>
        </section>

        <GoToBulmaIo path="elements"/>
    }
}
