use leptos::*;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{
    BasicBlock, BasicBox, BasicButtons, BasicIcon, BasicTable, BasicTitle, ButtonColors,
    ButtonSizes, ButtonStates, RustCodeExample,
};
use crate::i18n::{t, use_i18n};

#[component]
pub fn ElementsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, elements)/>

        <BTitle is=3>{t!(i18n, elements)}</BTitle>

        <BSection>
            <BTitle is=4>"Block"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_block"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicBlock/>

            <GoToDocsRs path="elements/fn.BBlock"/>
        </BSection>

        <BSection>
            <BTitle is=4>"Box"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_box"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicBox/>

            <GoToDocsRs path="elements/fn.BBox"/>
        </BSection>

        <BSection>
            <BTitle id="button" is=4>
                "Button"
            </BTitle>

            <BBlock>"Basic example:"</BBlock>

            <RustCodeExample name="basic_buttons"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicButtons/>

            <BTitle is=5>"Colors"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="button_colors"/>

            <BBlock>"See it in action:"</BBlock>

            <ButtonColors/>

            <BTitle is=5>"Sizes"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="button_sizes"/>

            <BBlock>"See it in action:"</BBlock>

            <ButtonSizes/>

            <BTitle is=5>"States"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="button_states"/>

            <BBlock>"See it in action:"</BBlock>

            <ButtonStates/>

            <GoToDocsRs path="elements/fn.BButton"/>
        </BSection>

        <BSection>
            <BTitle is=4>"Icon"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_icon"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicIcon/>

            <GoToDocsRs path="elements/fn.BIcon"/>
        </BSection>

        <BSection>
            <BTitle is=4>"Table"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_table"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicTable/>

            <GoToDocsRs path="elements/fn.BTable"/>
        </BSection>

        <BSection>
            <BTitle is=4>"Title"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_title"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicTitle/>

            <GoToDocsRs path="elements/fn.BTitle"/>
        </BSection>

        <GoToBulmaIo path="elements"/>
    }
}
