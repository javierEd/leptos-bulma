use indoc::indoc;
use leptos::*;

use crate::components::{CodeBlock, PageTitle};
use crate::i18n::{t, use_i18n};

#[component]
pub fn GuidesPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, guides)/>

        <h2 class="title is-3">{t!(i18n, guides)}</h2>

        <section class="section">
            <h3 class="title is-4">"How to install"</h3>

            <p class="block">"Run the following command to add the crate to your Leptos application:"</p>

            <CodeBlock language="bash">"cargo add leptos-bulma"</CodeBlock>

            <p class="block">"Or add the following line to your " <code>"Cargo.toml"</code> " file:"</p>

            <CodeBlock>r#"leptos-bulma = "0.0.0-alpha.8""#</CodeBlock>

            <p class="block">"Then add the following code to your " <code>"build.rs"</code> " file:"</p>

            <CodeBlock language="rust">
                {indoc!(
                    r#"fn main() {
                    // ···
                    leptos_bulma::LeptosBulma::build("./style");
                    // ···
                }"#
                )}

            </CodeBlock>

            <p class="block">"Use " <code>"leptos-bulma.css"</code> " in your stylesheet:"</p>

            <CodeBlock language="css">r#"@use "leptos-bulma.css";"#</CodeBlock>

            <p class="block">
                "And finally, add " <code>"leptos-bulma.css"</code> " to your " <code>".gitignore"</code> " file:"
            </p>

            <CodeBlock>"/style/leptos-bulma.css"</CodeBlock>
        </section>
    }
}
