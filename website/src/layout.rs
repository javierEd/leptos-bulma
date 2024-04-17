use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem,
    BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BButton, BButtons, BIcon};
use leptos_bulma::icons::icondata_fa;

use crate::app::use_theme_context;
use crate::i18n::{t, use_i18n, Locale};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let i18n = use_i18n();
    let theme = use_theme_context();
    let theme_is_dark = create_rw_signal(theme.is_dark());
    let theme_is_light = create_rw_signal(theme.is_light());
    let theme_is_system = create_rw_signal(theme.is_system());
    let burger_is_active = create_rw_signal(false);

    create_effect(move |_| {
        theme.get();
        theme_is_dark.set(theme.is_dark());
        theme_is_light.set(theme.is_light());
        theme_is_system.set(theme.is_system());
    });

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
                        <div class="title is-5">"Leptos Bulma"</div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItem href="/guides">{t!(i18n, guides)}</BNavbarItem>
                    <BNavbarItem href="/elements">{t!(i18n, elements)}</BNavbarItem>
                    <BNavbarItem href="/components">{t!(i18n, components)}</BNavbarItem>
                    <BNavbarItem href="/form">{t!(i18n, form)}</BNavbarItem>
                    <BNavbarItem href="/columns">{t!(i18n, columns)}</BNavbarItem>
                    <BNavbarItem href="/layout">{t!(i18n, layout)}</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <a
                        class="navbar-item"
                        href="https://github.com/javierEd/leptos-bulma"
                        target="_blank"
                        title="GitHub"
                    >
                        <BIcon is_large=true icon=icondata_fa::FaGithubBrands/>
                    </a>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>

        <main class="container">
            <div class="m-5">{children()}</div>
        </main>

        <footer class="footer">
            <div class="content container">
                <BColumns>
                    <BColumn>
                        <div class="is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, this_website_was_made_with)}
                            <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                                <picture>
                                    <source
                                        srcset="/images/leptos-logo-light.svg"
                                        media="(prefers-color-scheme: dark)"
                                    />
                                    <img src="/images/leptos-logo.svg" alt="Leptos" width="100"/>
                                </picture>
                            </a> & <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
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
                                    <source
                                        srcset="/images/github-logo-light.svg"
                                        media="(prefers-color-scheme: dark)"
                                    />
                                    <img src="/images/github-logo.svg" alt="GitHub" width="100"/>
                                </picture>
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow">
                        <BDropdown
                            class="mb-4"
                            is_right=true
                            is_up=true
                            trigger=move || {
                                view! { <span class="has-text-weight-bold">{t!(i18n, change_language)} " ▲"</span> }
                            }
                        >

                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::en)>"English"</BDropdownItem>
                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::es)>"Español"</BDropdownItem>
                        </BDropdown>

                        <BButtons has_addons=true>
                            <BButton
                                class="ml-auto"
                                title="System theme"
                                is_active=theme_is_system
                                on:click=move |_| theme.set_system()
                            >
                                <BIcon is_scaled=false icon=icondata_fa::FaDesktopSolid/>
                            </BButton>
                            <BButton title="Light theme" is_active=theme_is_light on:click=move |_| theme.set_light()>
                                <BIcon is_scaled=false icon=icondata_fa::FaSunSolid/>
                            </BButton>
                            <BButton title="Dark theme" is_active=theme_is_dark on:click=move |_| theme.set_dark()>
                                <BIcon is_scaled=false icon=icondata_fa::FaMoonSolid/>
                            </BButton>
                        </BButtons>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}
