use leptos::*;

mod b_checkbox;
mod b_checkbox_field;
mod b_file;
mod b_input;
mod b_password_field;
mod b_select;
mod b_select_field;
mod b_text_field;
mod b_textarea;
mod b_textarea_field;

pub use b_checkbox::*;
pub use b_checkbox_field::*;
pub use b_file::*;
pub use b_input::*;
pub use b_password_field::*;
pub use b_select::*;
pub use b_select_field::*;
pub use b_text_field::*;
pub use b_textarea::*;
pub use b_textarea_field::*;

#[component]
pub fn BControl(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("control {}", class)>{children()}</div> }
}

#[component]
pub fn BField(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("field {}", class)>{children()}</div> }
}

#[component]
pub fn BHelp(#[prop(default = "")] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("help {}", class)>{children()}</div> }
}

#[component]
pub fn BLabel(
    #[prop(optional, into)] for_id: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <label class="label" for=for_id>
            {children()}
        </label>
    }
}
