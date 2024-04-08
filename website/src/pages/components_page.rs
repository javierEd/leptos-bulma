use leptos::*;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicDropdown, BasicModal, RustCodeExample};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, components)/>

        <h2 class="title is-3">{t!(i18n, components)}</h2>

        <section class="section">
            <h3 class="title is-4">"Dropdown"</h3>

            <p class="block">"Example:"</p>

            <RustCodeExample name="basic_dropdown"/>

            <p class="block">"See it in action:"</p>

            <BasicDropdown/>

            <GoToDocsRs path="components/fn.BDropdown"/>
        </section>

        <section class="section">
            <h3 class="title is-4">"Modal"</h3>

            <p class="block">"Example:"</p>

            <RustCodeExample name="basic_modal"/>

            <p class="block">"See it in action:"</p>

            <BasicModal/>

            <GoToDocsRs path="components/fn.BModal"/>
        </section>

        <GoToBulmaIo path="components"/>
    }
}
