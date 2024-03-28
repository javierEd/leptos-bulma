mod app;
mod components;
mod layout;
mod pages;

#[cfg(feature = "ssr")]
mod server;

leptos_i18n::load_locales!();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(app::App);
}
