use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem};

use crate::components::PageTitle;
use crate::i18n::{t, use_i18n};

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, components)/>

        <h2 class="title is-3">{t!(i18n, components)}</h2>

        <section class="section">
            <h3 class="title is-4">Dropdown</h3>

            <p class="block">"Example:"</p>

            <pre class="block"><code>
r#"use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem};

#[component]
pub fn MyDropdown() -> impl IntoView {
    view! {
        <BDropdown
            class="m-4"
            trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }
        >
            <BDropdownItem>"Item"</BDropdownItem>
            <BDropdownDivider />
            <BDropdownItem>"Item"</BDropdownItem>
        </BDropdown>
    }
}"#
            </code></pre>

            <p class="block">"See it in action:"</p>

            <BDropdown
                class="block"
                trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }
            >
                <BDropdownItem>"Item"</BDropdownItem>
                <BDropdownDivider/>
                <BDropdownItem>"Item"</BDropdownItem>
            </BDropdown>
        </section>

        <section class="section">
            "To get more information about Bulma components, you can check the official documentation: "
            <a href="https://bulma.io/documentation/components" target="_blank">"bulma.io/documentation/components"</a>
        </section>
    }
}
