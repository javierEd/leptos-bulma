use leptos::*;

#[component]
pub fn BBox(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("box {}", class.get())>{children()}</div> }
}
