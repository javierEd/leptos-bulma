use icondata_core::IconData;
use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BButton, BButtons, BIcon};
use leptos_bulma::enums::{BSize, BState};
use leptos_bulma::icons::icondata_fa;
use leptos_use::ColorMode;

use crate::app::use_app_color_mode;
use crate::i18n::{t, use_i18n, Locale};

fn color_mode_options() -> [(&'static str, &'static IconData, ColorMode); 3] {
    [
        ("System theme", icondata_fa::FaDesktopSolid, ColorMode::Auto),
        ("Light theme", icondata_fa::FaSunSolid, ColorMode::Light),
        ("Dark theme", icondata_fa::FaMoonSolid, ColorMode::Dark),
    ]
}

#[component]
fn ImageColorModes(
    dark_src: &'static str,
    light_src: &'static str,
    alt: &'static str,
    width: i8,
) -> impl IntoView {
    let color_mode = use_app_color_mode().mode;

    view! {
        <picture>
            <Show when=move || [ColorMode::Dark, ColorMode::Auto].contains(&color_mode.get())>
                <source srcset=dark_src media="(prefers-color-scheme: dark)"/>
            </Show>

            <Show when=move || [ColorMode::Light, ColorMode::Auto].contains(&color_mode.get())>
                <source srcset=light_src media="(prefers-color-scheme: light)"/>
            </Show>

            <img
                src=move || { if color_mode.get() == ColorMode::Dark { dark_src } else { light_src } }
                alt=alt
                width=width
            />
        </picture>
    }
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let i18n = use_i18n();
    let app_color_mode = use_app_color_mode();
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
                        <div class="title is-5">"Leptos Bulma"</div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItemDropdown is_hoverable=true href="/guides" trigger=move || t!(i18n, guides)>
                        <BNavbarItem href="/guides#how-to-install-ssr">
                            {t!(i18n, how_to_install_in_leptos_ssr)}
                        </BNavbarItem>
                        <BNavbarItem href="/guides#how-to-install-csr">
                            {t!(i18n, how_to_install_in_leptos_csr)}
                        </BNavbarItem>
                        <BNavbarItem href="/guides#customization">{t!(i18n, customization)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItemDropdown is_hoverable=true href="/elements" trigger=move || t!(i18n, elements)>
                        <BNavbarItem href="/elements/button">{t!(i18n, button)}</BNavbarItem>
                        <BNavbarItem href="/elements/icon">{t!(i18n, icon)}</BNavbarItem>
                        <BNavbarItem href="/elements/tag">{t!(i18n, tag)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItemDropdown is_hoverable=true href="/components" trigger=move || t!(i18n, components)>
                        <BNavbarItem href="/components/breadcrumb">{t!(i18n, breadcrumb)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItem href="/form">{t!(i18n, form)}</BNavbarItem>
                    <BNavbarItem href="/columns">{t!(i18n, columns)}</BNavbarItem>
                    <BNavbarItem href="/layout">{t!(i18n, layout)}</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem href="https://github.com/javierEd/leptos-bulma" target="_blank" title="GitHub">
                        <BIcon icon=icondata_fa::FaGithubBrands size=BSize::Large is_scaled=true/>
                    </BNavbarItem>
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
                                <ImageColorModes
                                    dark_src="/images/leptos-logo-light.svg"
                                    light_src="/images/leptos-logo.svg"
                                    alt="Leptos"
                                    width=100
                                />
                            </a> & <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                                <ImageColorModes
                                    dark_src="/images/bulma-logo-light.svg"
                                    light_src="/images/bulma-logo.svg"
                                    alt="Bulma"
                                    width=100
                                />
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
                                <ImageColorModes
                                    dark_src="/images/github-logo-light.svg"
                                    light_src="/images/github-logo.svg"
                                    alt="GitHub"
                                    width=100
                                />
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow has-text-right">
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
                            <For
                                each=color_mode_options
                                key=|(_, _, mode)| mode.clone()
                                children=move |(title, icon, mode)| {
                                    let mode_clone = mode.clone();
                                    let assign_button_state = move || {
                                        let color_mode = app_color_mode.mode.get();
                                        if color_mode == mode_clone { BState::Active } else { BState::Default }
                                    };
                                    let button_state = create_rw_signal(assign_button_state());
                                    create_effect(move |_| {
                                        button_state.set(assign_button_state());
                                    });
                                    view! {
                                        <BButton
                                            title=title
                                            on:click=move |_| app_color_mode.set_mode.set(mode.clone())
                                            state=button_state
                                        >
                                            <BIcon is_scaled=false icon=icon/>
                                        </BButton>
                                    }
                                }
                            />

                        </BButtons>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}
