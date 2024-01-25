use leptos::*;

#[component]
pub fn BBox(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <div class=format!("box {}", class)>{children()}</div> }
}
