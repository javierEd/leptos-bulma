use leptos::*;

#[component]
pub fn BBox(children: Children) -> impl IntoView {
    view! {
        <div class="box">{children()}</div>
    }
}
