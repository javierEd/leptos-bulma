use leptos::*;

#[component]
pub fn BSection(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <section class=format!("section {}", class.get())>{children()}</section> }
}
