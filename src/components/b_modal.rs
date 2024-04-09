use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn BModal(
    children: ChildrenFn,
    #[prop(default = true)] has_background: bool,
    #[prop(default = true)] has_close_button: bool,
    #[prop(default = true)] is_dismissible: bool,
    #[prop(optional, into)] on_close: Option<Callback<MouseEvent>>,
    is_active: RwSignal<bool>,
) -> impl IntoView {
    let close_modal = move |event| {
        is_active.set(false);

        if let Some(oc) = on_close {
            oc.call(event);
        }
    };

    view! {
        <Show when=move || is_active.get()>
            <div class="modal is-active">
                <Show when=move || has_background>
                    <div
                        class="modal-background"
                        on:click=move |event| {
                            if is_dismissible {
                                close_modal(event);
                            }
                        }
                    >
                    </div>
                </Show>
                {children()}

                <Show when=move || has_close_button>
                    <BModalClose on:click=close_modal/>
                </Show>
            </div>
        </Show>
    }
}

#[component]
pub fn BModalClose() -> impl IntoView {
    view! { <a class="modal-close is-large"></a> }
}

#[component]
pub fn BModalContent(children: Children) -> impl IntoView {
    view! { <div class="modal-content">{children()}</div> }
}
