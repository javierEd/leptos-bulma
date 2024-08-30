use leptos::*;
use leptos_bulma::elements::{BButton, BNotification};

#[component]
pub fn BasicNotification() -> impl IntoView {
    let is_active = create_rw_signal(true);

    view! {
        <BNotification is_active=is_active>
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit lorem ipsum dolor. "
            <strong>"Pellentesque risus mi"</strong>", tempus quis placerat ut, porta nec
            nulla. Vestibulum rhoncus ac ex sit amet fringilla. Nullam gravida purus diam,
            et dictum "<a>"felis venenatis"</a>" efficitur."
        </BNotification>

        <Show when=move || !is_active.get()>
            <BButton is_fullwidth=true on:click=move |_| is_active.set(true)>
                "Show notification"
            </BButton>
        </Show>
    }
}
