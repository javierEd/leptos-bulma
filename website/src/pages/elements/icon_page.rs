use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicIcons, IconPackages, IconSizes, RustCodeExample};
use crate::i18n::{t, use_i18n};

#[component]
pub fn IconPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, icon)/>

        <BTitle is=3>{t!(i18n, icon)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_icons"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicIcons/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, sizes)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="icon_sizes"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <IconSizes/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, packages)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="icon_packages"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BBlock>
                <IconPackages/>
            </BBlock>

            <BBlock>
                "If you want to find more about the icons available, you can check the following link: "
                <a href="https://carloskiki.github.io/icondata/" target="_blank">
                    "carloskiki.github.io/icondata"
                </a>
            </BBlock>
        </BSection>

        <GoToDocsRs path="elements/fn.BIcon"/>

        <GoToBulmaIo path="elements/icon"/>
    }
}
