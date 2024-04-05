use indoc::indoc;
use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem, BModal, BModalContent};
use leptos_bulma::elements::BBox;

use crate::components::{CodeBlock, GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let i18n = use_i18n();

    let modal_is_active = create_rw_signal(false);

    view! {
        <PageTitle text=t!(i18n, components)/>

        <h2 class="title is-3">{t!(i18n, components)}</h2>

        <section class="section">
            <h3 class="title is-4">"Dropdown"</h3>

            <p class="block">"Example:"</p>

            <CodeBlock>
                {indoc!(
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
                                <BDropdownDivider/>
                                <BDropdownItem>"Item"</BDropdownItem>
                            </BDropdown>
                        }
                    }"#
                )}

            </CodeBlock>

            <p class="block">"See it in action:"</p>

            <BDropdown class="block" trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }>
                <BDropdownItem>"Item"</BDropdownItem>
                <BDropdownDivider/>
                <BDropdownItem>"Item"</BDropdownItem>
            </BDropdown>

            <GoToDocsRs path="components/fn.BDropdown"/>
        </section>

        <section class="section">
            <h3 class="title is-4">"Modal"</h3>

            <p class="block">"Example:"</p>

            <CodeBlock>
                {indoc!(
                    r#"use leptos::*;
                    use leptos_bulma::components::{BModal, BModalContent};
                    use leptos_bulma::elements::BBox;

                    #[component]
                    pub fn MyModal() -> impl IntoView {
                        let modal_is_active = create_rw_signal(false);

                        view !{
                            <a class="button" on:click=move |_| modal_is_active.set(true)>
                                "Open modal"
                            </a>

                            <BModal is_active=modal_is_active>
                                <BModalContent>
                                    <BBox class="has-text-centered">"Hello, World!"</BBox>
                                </BModalContent>
                            </BModal>
                        }
                    }"#
                )}

            </CodeBlock>

            <p class="block">"See it in action:"</p>

            <a class="button block" on:click=move |_| modal_is_active.set(true)>
                "Open modal"
            </a>

            <BModal is_active=modal_is_active>
                <BModalContent>
                    <BBox class="has-text-centered">"Hello, World!"</BBox>
                </BModalContent>
            </BModal>

            <GoToDocsRs path="components/fn.BModal"/>
        </section>

        <GoToBulmaIo path="components"/>
    }
}
