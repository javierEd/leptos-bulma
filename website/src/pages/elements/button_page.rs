use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicButtons, ButtonColors, ButtonSizes, ButtonStates, RustCodeExample};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ButtonPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, button)/>

        <BTitle is=3>{t!(i18n, button)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_buttons"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicButtons/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, colors)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="button_colors"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ButtonColors/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, sizes)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="button_sizes"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ButtonSizes/>
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, states)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="button_states"/>

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <ButtonStates/>
        </BSection>

        <GoToDocsRs path="elements/fn.BButton"/>

        <GoToBulmaIo path="elements/button"/>
    }
}
