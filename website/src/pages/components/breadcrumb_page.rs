use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{
    BasicBreadcrumb, BreadcrumbAlignments, BreadcrumbSeparators, BreadcrumbSizes, RustCodeExample,
};
use crate::i18n::{t, use_i18n};

#[component]
pub fn BreadcrumbPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, breadcrumb)/>

        <BTitle is=3>{t!(i18n, breadcrumb)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_breadcrumb"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicBreadcrumb/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, alignment)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="breadcrumb_alignments"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BreadcrumbAlignments/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, separators)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="breadcrumb_separators"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BreadcrumbSeparators/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, sizes)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="breadcrumb_sizes"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BreadcrumbSizes/>
        </BSection>

        <GoToDocsRs path="components/fn.BBreadcrumb"/>

        <GoToBulmaIo path="components/breadcrumb"/>
    }
}
