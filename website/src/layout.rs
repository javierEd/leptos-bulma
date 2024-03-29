use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem,
    BNavbarMenu, BNavbarStart,
};

use crate::i18n::{t, use_i18n, Locale};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let i18n = use_i18n();

    let burger_is_active = create_rw_signal(false);

    view! {
        <BNavbar class="has-shadow">
            <BNavbarBrand>
                <BNavbarItem class="media mb-0 is-align-items-center" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="title is-5 has-text-light">"Leptos Bulma"</div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItem href="/guides">{t!(i18n, guides)}</BNavbarItem>
                    <BNavbarItem href="/components">{t!(i18n, components)}</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <a class="navbar-item" href="https://github.com/javierEd/leptos-bulma" target="_blank" title="GitHub">
                        <span class="icon mx-1">
                            <img src="/images/github-icon-light.svg" alt="GitHub"/>
                        </span>
                    </a>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>

        <main class="container"><div class="m-5">{children()}</div></main>

        <footer class="footer">
            <div class="content container">
                <BColumns>
                    <BColumn>
                        <div class="is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, this_website_was_made_with)}
                            <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                                <picture>
                                    <source srcset="/images/leptos-logo-light.svg" media="(prefers-color-scheme: dark)"/>
                                    <img src="/images/leptos-logo.svg" alt="Leptos" width="100"/>
                                </picture>
                            </a>
                            &
                            <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                                <picture>
                                    <source srcset="/images/bulma-logo-light.svg" media="(prefers-color-scheme: dark)"/>
                                    <img src="/images/bulma-logo.svg" alt="Bulma" width="100"/>
                                </picture>
                            </a>
                        </div>
                        <div class="mt-3 is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, and_you_can_see_the_source_code_at)}
                            <a
                                class="mx-3"
                                href="https://github.com/javierEd/leptos-bulma/blob/main/website"
                                target="_blank"
                                title="Go to GitHub"
                            >
                                <picture>
                                    <source srcset="/images/github-logo-light.svg" media="(prefers-color-scheme: dark)"/>
                                    <img src="/images/github-logo.svg" alt="GitHub" width="100"/>
                                </picture>
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow">
                        <BDropdown
                            trigger=|| view! {
                                <span class="has-text-weight-bold">{t!(i18n, change_language)}" ▼"</span>
                            }
                        >
                            <BDropdownItem on_click=move|_| i18n.set_locale(Locale::en)>"English"</BDropdownItem>
                            <BDropdownItem on_click=move|_| i18n.set_locale(Locale::es)>"Español"</BDropdownItem>
                        </BDropdown>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}
