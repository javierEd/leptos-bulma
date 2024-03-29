use leptos::*;

use crate::components::PageTitle;
use crate::i18n::{t, use_i18n};

#[component]
pub fn GuidesPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, guides)/>

        <h3 class="title is-4">"How to install"</h3>

        <p class="block">"Run the following command to add the crate to your Leptos application:"</p>

        <pre class="block"><code>"cargo add leptos-bulma"</code></pre>

        <p class="block">"Or add the following line to your "<code>"Cargo.toml"</code>" file:"</p>

        <pre class="block"><code>"leptos-bulma = \"0.0.0-alpha.7\""</code></pre>

        <p class="block">"Then add the following code to your "<code>"build.rs"</code>" file:"</p>

        <pre class="block"><code>
"fn main() {
    // ···
    leptos_bulma::LeptosBulma::build(\"./style\");
    // ···
}"
        </code></pre>

        <p class="block">"Use "<code>"leptos-bulma.css"</code>" in your stylesheet:"</p>

        <pre class="block"><code>"@use \"leptos-bulma.css\";"</code></pre>

        <p class="block">
            "And finally, add "<code>"leptos-bulma.css"</code>" to your "<code>".gitignore"</code>" file:"
        </p>

        <pre class="block"><code>"/style/leptos-bulma.css"</code></pre>
    }
}
