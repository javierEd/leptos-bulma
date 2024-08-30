use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{
    BasicProgress, ProgressColors, ProgressIndeterminate, ProgressSizes, RustCodeExample,
};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ProgressPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, progress) />

        <BTitle is=3>{t!(i18n, progress)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_progress" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicProgress />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, colors)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="progress_colors" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ProgressColors />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, sizes)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="progress_sizes" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ProgressSizes />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, indeterminate)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="progress_indeterminate" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ProgressIndeterminate />
        </BSection>

        <GoToDocsRs path="elements/fn.BProgress" />

        <GoToBulmaIo path="elements/progress" />
    }
}
