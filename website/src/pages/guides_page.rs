use leptos::*;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{CodeBlock, PageTitle};
use crate::i18n::{t, use_i18n};

#[component]
pub fn GuidesPage() -> impl IntoView {
    let i18n = use_i18n();

    let cargo_pkg_version = env!("CARGO_PKG_VERSION");

    let cargo_toml = "[build-dependencies]\n# ···\nleptos-bulma = { version = \"".to_owned()
            + cargo_pkg_version
            + "\", default-features = false, features = [\"build-script\"] }\n\n[dependencies]\n# ···\nleptos-bulma = \""
            + cargo_pkg_version
            + "\"";

    let build_rs = "fn main() {\n    // ···\n    leptos_bulma::build(\"./style \");\n}";

    view! {
        <PageTitle text=t!(i18n, guides)/>

        <BTitle is=3>{t!(i18n, guides)}</BTitle>

        <BSection>
            <BTitle is=4>"How to install"</BTitle>

            <BBlock>"Add the following lines to your " <code>"Cargo.toml"</code> " file:"</BBlock>

            <CodeBlock>{cargo_toml}</CodeBlock>

            <BBlock>"Then add the following code to your " <code>"build.rs"</code> " file:"</BBlock>

            <CodeBlock language="rust">{build_rs}</CodeBlock>

            <BBlock>"Use " <code>"leptos-bulma.scss"</code> " in your stylesheet:"</BBlock>

            <CodeBlock language="css">r#"@use "leptos-bulma.scss";"#</CodeBlock>

            <BBlock>
                "And finally, add " <code>"leptos-bulma.scss"</code> " to your " <code>".gitignore"</code> " file:"
            </BBlock>

            <CodeBlock>"/style/leptos-bulma.scss"</CodeBlock>
        </BSection>
    }
}
