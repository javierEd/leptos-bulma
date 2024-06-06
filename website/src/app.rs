use leptos::*;
use leptos_i18n::Locale;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::utils::FromToStringCodec;
use leptos_use::{
    use_color_mode_with_options, use_cookie, UseColorModeOptions, UseColorModeReturn,
};

use crate::i18n::provide_i18n_context;
use crate::layout::Layout;
use crate::pages::*;

pub fn use_app_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(
        UseColorModeOptions::default()
            .cookie_enabled(true)
            .cookie_name("pref_color_mode")
            .attribute("data-theme")
            .emit_auto(true)
            .transition_enabled(true),
    )
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let i18n = provide_i18n_context();

    let (locale_cookie, _) = use_cookie::<String, FromToStringCodec>("i18n_pref_locale");

    i18n.set_locale(
        Locale::from_str(&locale_cookie.get().unwrap_or("en".to_owned())).unwrap_or_default(),
    );

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        // sets the favicon
        <Link rel="icon" href="/images/favicon.png"/>

        // sets the document title
        <Title text="Leptos Bulma - A Leptos component library based on Bulma CSS framework"/>

        <div class="loading-overlay" class:is-done=leptos_dom::is_browser></div>

        // content for this welcome page
        <Router trailing_slash=TrailingSlash::Redirect>
            <Layout>
                <Routes>
                    <Route path="/columns" view=ColumnsPage/>
                    <Route path="/components" view=ComponentsPage/>
                    <Route path="/elements" view=ElementsPage/>
                    <Route path="/elements/tag" view=elements::TagPage/>
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
