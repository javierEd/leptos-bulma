use leptos::*;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{CodeBlock, PageTitle};
use crate::i18n::{t, use_i18n};

#[component]
pub fn GuidesPage() -> impl IntoView {
    let i18n = use_i18n();

    let cargo_pkg_version = env!("CARGO_PKG_VERSION");

    let ssr_cargo_toml = "[build-dependencies]\n# ···\nleptos-bulma = { version = \"".to_owned()
        + cargo_pkg_version
        + "\", default-features = false, features = [\"build-script\"] }\n\n[dependencies]\n# ···\nleptos-bulma = \""
        + cargo_pkg_version
        + "\"\n\n[features]\n# ···\nssr = [\n    # ···\n    \"leptos-bulma/ssr\"\n]";

    let build_rs = "fn main() {\n    // ···\n    leptos_bulma::build(\"./style\");\n}";

    let csr_cargo_toml = "[dependencies]\n# ···\nleptos-bulma = { version = \"".to_owned()
        + cargo_pkg_version
        + ", features = [\"build-script\"] }";

    let csr_index_html = "<!-- Add leptos-bulma.scss -->\n".to_owned()
        + r#"<link data-trunk rel="scss" rel="stylesheet" href="style/leptos-bulma.scss"/>"# + "\n"
        + "<!-- Since you added another binary, you should modify the following line and add the default target name -->\n"
        + r#"<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs data-target-name="<PACKAGE-NAME>"/>"#;

    let trunk_toml = "[watch]\n# ···\nignore = [\"style\"]\n\n[[hooks]]\nstage = \"pre_build\"\n"
        .to_owned()
        + r#"command = "cargo""#
        + "\n"
        + r#"command_arguments = ["run", "--bin", "build-leptos-bulma"]"#;

    let customize_sass = "// Set your brand colors\n$purple: #8a4d76;\n$pink: #fa7c91;\n$brown: #757763;\n".to_owned()
        + "$beige-light: #d0d1cd;\n$beige-lighter: #eff0eb;\n\n@use \"leptos-bulma.scss\" with (\n"
        + "    $grey-dark: $brown,\n    $grey-light: $beige-light,\n    $primary: $purple,\n    $link: $pink,\n"
        + "    $control-border-width: 2px,\n    $input-shadow: none\n);";

    view! {
        <PageTitle text=t!(i18n, guides)/>

        <BTitle is=3>{t!(i18n, guides)}</BTitle>

        <BSection>
            <BTitle id="how-to-install-ssr" is=4>
                {t!(i18n, how_to_install_in_leptos_ssr)}
                " (Actix, Axum or Spin)"
            </BTitle>

            <BBlock>
                "Before you start, you should have already generated your Leptos application with one of the following templates:"
                <br/> "- " <a href="https://github.com/leptos-rs/start" target="_blank">
                    "github.com/leptos-rs/start"
                </a> <br/> "- " <a href="https://github.com/leptos-rs/start-axum" target="_blank">
                    "github.com/leptos-rs/start-axum"
                </a> <br/> "- " <a href="https://github.com/leptos-rs/start-spin" target="_blank">
                    "github.com/leptos-rs/start-spin"
                </a>
            </BBlock>

            <BBlock>"Add the following lines to your " <code>"Cargo.toml"</code> " file:"</BBlock>

            <CodeBlock>{ssr_cargo_toml}</CodeBlock>

            <BBlock>"Then add the following code to your " <code>"build.rs"</code> " file:"</BBlock>

            <CodeBlock language="rust">{build_rs}</CodeBlock>

            <BBlock>"Use " <code>"leptos-bulma.scss"</code> " in your stylesheet:"</BBlock>

            <CodeBlock language="css">r#"@use "leptos-bulma.scss";"#</CodeBlock>

            <BBlock>
                "And finally, add " <code>"leptos-bulma.scss"</code> " to your " <code>".gitignore"</code> " file:"
            </BBlock>

            <CodeBlock>"/style/leptos-bulma.scss"</CodeBlock>
        </BSection>

        <BSection>
            <BTitle id="how-to-install-csr" is=4>
                {t!(i18n, how_to_install_in_leptos_csr)}
                " (Trunk)"
            </BTitle>

            <BBlock>
                "In case you generated your Leptos application with the following template:" <br/> "- "
                <a href="https://github.com/leptos-rs/start-trunk" target="_blank">
                    "github.com/leptos-rs/start-trunk"
                </a>
            </BBlock>

            <BBlock>"Add the following line to your " <code>"Cargo.toml"</code> " file:"</BBlock>

            <CodeBlock>{csr_cargo_toml}</CodeBlock>

            <BBlock>
                "Then create a file in the path" <code>"src/bin/build-leptos-bulma.rs"</code>
                " and add the following code:"
            </BBlock>

            <CodeBlock language="rust">{build_rs}</CodeBlock>

            <BBlock>"Run the following command to generate the " <code>"leptos-bulma.scss"</code> " file:"</BBlock>

            <CodeBlock language="bash">"cargo run --bin build-leptos-bulma"</CodeBlock>

            <BBlock>"Modify your " <code>"index.html"</code> " with the following lines:"</BBlock>

            <CodeBlock language="html">{csr_index_html}</CodeBlock>

            <BBlock>
                "Optionally you can add the following lines to your " <code>"Trunk.toml"</code>
                ", but be aware that the directory" <code>"style"</code> " will be ignored from watch:"
            </BBlock>

            <CodeBlock>{trunk_toml}</CodeBlock>

            <BBlock>
                "And finally, add " <code>"leptos-bulma.scss"</code> " to your " <code>".gitignore"</code> " file:"
            </BBlock>

            <CodeBlock>"/style/leptos-bulma.scss"</CodeBlock>
        </BSection>

        <BSection>
            <BTitle id="customization" is=4>
                {t!(i18n, customization)}
            </BTitle>

            <BTitle id="customization" is=5>
                "Customize with Sass"
            </BTitle>

            <BBlock>
                "You can overwrite Bulma’s Sass variables with your own values by using the " <code>"with"</code>
                " in your stylesheet:"
            </BBlock>

            <CodeBlock language="scss">{customize_sass}</CodeBlock>

            <BBlock>
                "To check the available variables, go to: "
                <a href="https://bulma.io/documentation/customize/list-of-sass-variables" target="_blank">
                    "bulma.io/documentation/customize/list-of-sass-variables"
                </a> "."
            </BBlock>
        </BSection>
    }
}
