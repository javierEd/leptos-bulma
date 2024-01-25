use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn BModal(
    children: Children,
    #[prop(default = true)] has_background: bool,
    is_active: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="modal" class:is-active=is_active>
            <Show when=move || has_background>
                <div class="modal-background"></div>
            </Show>
            {children()}
        </div>
    }
}

#[component]
pub fn BModalClose<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <a class="modal-close is-large" on:click=on_click></a> }
}

#[component]
pub fn BModalContent(children: Children) -> impl IntoView {
    view! { <div class="modal-content">{children()}</div> }
}
