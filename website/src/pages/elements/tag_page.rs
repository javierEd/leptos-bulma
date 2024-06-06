use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicTags, RustCodeExample, TagAddons, TagColors, TagSizes};
use crate::i18n::{t, use_i18n};

#[component]
pub fn TagPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, tag)/>

        <BTitle is=3>{t!(i18n, tag)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_tags"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicTags/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, colors)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="tag_colors"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <TagColors/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, sizes)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="tag_sizes"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <TagSizes/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, addons)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="tag_addons"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <TagAddons/>
        </BSection>

        <GoToDocsRs path="elements/fn.BTag"/>

        <GoToBulmaIo path="elements/tag"/>
    }
}
