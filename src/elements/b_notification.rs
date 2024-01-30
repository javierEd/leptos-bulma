use leptos::*;

#[component]
pub fn BNotification(
    children: ChildrenFn,
    #[prop(default = "")] class: &'static str,
    is_active: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || is_active.get()>
            <div class=format!("notification {}", class)>
                <button class="delete" on:click=move |_| { is_active.set(false) }></button>
                {children()}
            </div>
        </Show>
    }
}
