use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn BModal(
    children: ChildrenFn,
    #[prop(default = true)] has_background: bool,
    #[prop(default = true)] has_close_button: bool,
    #[prop(default = true)] is_dismissible: bool,
    is_active: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || is_active.get()>
            <div class="modal is-active">
                <Show when=move || has_background>
                    <div
                        class="modal-background"
                        on:click=move |_| {
                            if is_dismissible {
                                is_active.set(false)
                            }
                        }
                    >
                    </div>
                </Show>
                {children()}

                <Show when=move || has_close_button>
                    <BModalClose on_click=move |_| is_active.set(false)/>
                </Show>
            </div>
        </Show>
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
