use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::components::CodeBlock;

mod basic_block;
mod basic_box;
mod basic_breadcrumb;
mod basic_buttons;
mod basic_columns;
mod basic_dropdown;
mod basic_icons;
mod basic_modal;
mod basic_navbar;
mod basic_notification;
mod basic_pagination;
mod basic_progress;
mod basic_section;
mod basic_table;
mod basic_tags;
mod basic_title;
mod breadcrumb_alignments;
mod breadcrumb_separators;
mod breadcrumb_sizes;
mod button_colors;
mod button_sizes;
mod button_states;
mod general_form;
mod icon_packages;
mod icon_sizes;
mod notification_colors;
mod notification_light_colors;
mod progress_colors;
mod progress_indeterminate;
mod progress_sizes;
mod tag_addons;
mod tag_colors;
mod tag_sizes;

pub use basic_block::BasicBlock;
pub use basic_box::BasicBox;
pub use basic_breadcrumb::BasicBreadcrumb;
pub use basic_buttons::BasicButtons;
pub use basic_columns::BasicColumns;
pub use basic_dropdown::BasicDropdown;
pub use basic_icons::BasicIcons;
pub use basic_modal::BasicModal;
pub use basic_navbar::BasicNavbar;
pub use basic_notification::BasicNotification;
pub use basic_pagination::BasicPagination;
pub use basic_progress::BasicProgress;
pub use basic_section::BasicSection;
pub use basic_table::BasicTable;
pub use basic_tags::BasicTags;
pub use basic_title::BasicTitle;
pub use breadcrumb_alignments::BreadcrumbAlignments;
pub use breadcrumb_separators::BreadcrumbSeparators;
pub use breadcrumb_sizes::BreadcrumbSizes;
pub use button_colors::ButtonColors;
pub use button_sizes::ButtonSizes;
pub use button_states::ButtonStates;
pub use general_form::GeneralForm;
pub use icon_packages::IconPackages;
pub use icon_sizes::IconSizes;
pub use notification_colors::NotificationColors;
pub use notification_light_colors::NotificationLightColors;
pub use progress_colors::ProgressColors;
pub use progress_indeterminate::ProgressIndeterminate;
pub use progress_sizes::ProgressSizes;
pub use tag_addons::TagAddons;
pub use tag_colors::TagColors;
pub use tag_sizes::TagSizes;

async fn get_code_example(name: &'static str) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    let code_example_path = format!("/examples/{name}.rs");
    let request = Request::new_with_str_and_init(&code_example_path, &opts)?;
    let response = JsFuture::from(window().fetch_with_request(&request))
        .await?
        .dyn_into::<Response>()?;
    JsFuture::from(response.text()?)
        .await?
        .as_string()
        .ok_or(JsValue::UNDEFINED)
}

#[component]
pub fn RustCodeExample(name: &'static str) -> impl IntoView {
    let resource = create_local_resource(move || name, get_code_example);

    view! {
        <Suspense fallback=move || {
            view! { <CodeBlock>"Loading..."</CodeBlock> }
        }>
            {resource
                .get()
                .and_then(|result| result.ok())
                .map(|code_example| view! { <CodeBlock language="rust">{code_example}</CodeBlock> })}

        </Suspense>
    }
}
