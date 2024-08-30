use leptos::*;

use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{
    BasicNotification, NotificationColors, NotificationLightColors, RustCodeExample,
};
use crate::i18n::{t, use_i18n};

#[component]
pub fn NotificationPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, notification) />

        <BTitle is=3>{t!(i18n, notification)}</BTitle>

        <BSection>
            <BTitle is=4>{t!(i18n, basic)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="basic_notification" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <BasicNotification />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, colors)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="notification_colors" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <NotificationColors />
        </BSection>

        <BSection>
            <BTitle is=4>{t!(i18n, light_colors)}</BTitle>

            <BBlock>{t!(i18n, code_sample)}</BBlock>

            <RustCodeExample name="notification_light_colors" />

            <BBlock>{t!(i18n, see_it_in_action)}</BBlock>

            <NotificationLightColors />
        </BSection>
        <GoToDocsRs path="elements/fn.BNotification" />

        <GoToBulmaIo path="elements/notification" />
    }
}
