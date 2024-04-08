use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::i18n::provide_i18n_context;
use crate::layout::Layout;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        // sets the favicon
        <Link rel="icon" href="/images/favicon.png"/>

        // sets the document title
        <Title text="Leptos Bulma - A Leptos component library based on Bulma CSS framework"/>

        // content for this welcome page
        <Router>
            <Layout>
                <Routes>
                    <Route path="/components" view=ComponentsPage/>
                    <Route path="/elements" view=ElementsPage/>
                    <Route path="/form" view=FormPage/>
                    <Route path="/guides" view=GuidesPage/>
                    <Route path="/" view=HomePage/>
                    <Route path="/*" view=NotFoundPage/>
                </Routes>
            </Layout>
        </Router>
    }
}
