use leptos::*;

#[component]
pub fn BBlock(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("block {}", class.get())>{children()}</div> }
}
