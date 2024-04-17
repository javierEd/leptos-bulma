use leptos::*;
use leptos_i18n::Locale;
use leptos_meta::*;
use leptos_router::*;

use crate::i18n::provide_i18n_context;
use crate::layout::Layout;
use crate::pages::*;

const LOCALE_COOKIE_NAME: &str = "i18n_pref_locale";
const THEME_COOKIE_NAME: &str = "pref_theme";

#[cfg(feature = "ssr")]
fn use_ssr_cookie(name: &'static str) -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use leptos_use::{use_cookie_with_options, UseCookieOptions};

    use_cookie_with_options::<String, leptos_use::utils::FromToStringCodec>(
        name,
        UseCookieOptions::default()
            .ssr_cookies_header_getter(|| {
                use_context::<leptos_spin::RequestParts>().and_then(|req| {
                    req.headers()
                        .iter()
                        .find(|(key, _)| key == "cookie")
                        .and_then(|(_, value)| String::from_utf8(value.to_vec()).ok())
                })
            })
            .ssr_set_cookie(|_| {}),
    )
}

#[cfg(feature = "ssr")]
fn use_locale_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use_ssr_cookie(LOCALE_COOKIE_NAME)
}

#[cfg(not(feature = "ssr"))]
fn use_locale_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>(LOCALE_COOKIE_NAME)
}

#[cfg(feature = "ssr")]
fn use_theme_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use_ssr_cookie(THEME_COOKIE_NAME)
}

#[cfg(not(feature = "ssr"))]
fn use_theme_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>(THEME_COOKIE_NAME)
}

#[derive(Clone, Copy)]
pub struct AppTheme(RwSignal<Option<String>>);

impl AppTheme {
    pub fn get(&self) -> Option<String> {
        self.0.get()
    }

    pub fn is_dark(&self) -> bool {
        self.0.with(|value| *value == Some("dark".to_owned()))
    }

    pub fn is_light(&self) -> bool {
        self.0.with(|value| *value == Some("light".to_owned()))
    }

    pub fn is_system(&self) -> bool {
        self.0.with(|value| value.is_none())
    }

    pub fn set_dark(&self) {
        self.0.set(Some("dark".to_owned()))
    }
    pub fn set_light(&self) {
        self.0.set(Some("light".to_owned()))
    }
    pub fn set_system(&self) {
        self.0.set(None)
    }
}

fn provide_theme_context() -> RwSignal<Option<String>> {
    let theme = create_rw_signal(None);
    let (theme_cookie, set_theme_cookie) = use_theme_cookie();

    theme.set(theme_cookie.get());

    provide_context(AppTheme(theme));

    create_effect(move |_| {
        set_theme_cookie.set(theme.get());
    });

    theme
}

pub fn use_theme_context() -> AppTheme {
    use_context::<AppTheme>().expect("Could not get theme context")
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let i18n = provide_i18n_context();

    let (locale_cookie, _) = use_locale_cookie();

    i18n.set_locale(
        Locale::from_str(&locale_cookie.get().unwrap_or("en".to_owned())).unwrap_or_default(),
    );

    let theme = provide_theme_context();

    view! {
        <Html attr:data-theme=theme/>

        <Stylesheet id="leptos" href="/pkg/website.css"/>

        // sets the favicon
        <Link rel="icon" href="/images/favicon.png"/>

        // sets the document title
        <Title text="Leptos Bulma - A Leptos component library based on Bulma CSS framework"/>

        // content for this welcome page
        <Router>
            <Layout>
                <Routes>
                    <Route path="/columns" view=ColumnsPage/>
                    <Route path="/components" view=ComponentsPage/>
                    <Route path="/elements" view=ElementsPage/>
                    <Route path="/form" view=FormPage/>
                    <Route path="/guides" view=GuidesPage/>
                    <Route path="/layout" view=LayoutPage/>
                    <Route path="/" view=HomePage/>
                    <Route path="/*" view=NotFoundPage/>
                </Routes>
            </Layout>
        </Router>
    }
}
