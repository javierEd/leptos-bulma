use leptos::*;
use leptos_bulma::elements::{BButton, BNotification};
use leptos_bulma::enums::BColor;

const NOTIFICATION_COLOR_OPTIONS: [BColor; 7] = [
    BColor::Danger,
    BColor::Default,
    BColor::Info,
    BColor::Link,
    BColor::Primary,
    BColor::Success,
    BColor::Warning,
];

#[component]
pub fn NotificationColors() -> impl IntoView {
    view! {
        <For
            each=move || NOTIFICATION_COLOR_OPTIONS
            key=|bcolor| bcolor.clone()
            children=move |bcolor| {
                let is_active = create_rw_signal(true);
                let color = store_value(bcolor.clone());
                view! {
                    <BNotification color=bcolor.clone() is_active=is_active>
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit lorem ipsum dolor. "
                        <strong>"Pellentesque risus mi"</strong>
                        ", tempus quis placerat ut, porta nec
                        nulla. Vestibulum rhoncus ac ex sit amet fringilla. Nullam gravida purus diam,
                        et dictum "
                        <a>"felis venenatis"</a>
                        " efficitur."
                    </BNotification>

                    <Show when=move || !is_active.get()>
                        <BButton class="block" is_fullwidth=true on:click=move |_| is_active.set(true)>
                            "Show "
                            {color.with_value(|bcolor| String::from(bcolor))}
                            " notification"
                        </BButton>
                    </Show>
                }
            }
        />
    }
}
