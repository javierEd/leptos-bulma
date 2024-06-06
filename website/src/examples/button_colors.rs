use leptos::*;
use leptos_bulma::elements::{BButton, BButtons};
use leptos_bulma::enums::BColor;

fn button_color_options() -> [BColor; 11] {
    [
        BColor::Custom("black".to_owned()),
        BColor::Custom("ghost".to_owned()),
        BColor::Custom("white".to_owned()),
        BColor::Danger,
        BColor::Default,
        BColor::Info,
        BColor::Link,
        BColor::Primary,
        BColor::Success,
        BColor::Text,
        BColor::Warning,
    ]
}

#[component]
pub fn ButtonColors() -> impl IntoView {
    let button_color = create_rw_signal(button_color_options().first().unwrap().clone());

    let button_on_click = move |_| {
        let mut color_index = button_color_options()
            .iter()
            .position(|value| *value == button_color.get())
            .unwrap()
            + 1;

        if color_index >= button_color_options().len() {
            color_index = 0;
        }

        button_color.set(button_color_options().get(color_index).unwrap().clone());
    };

    view! {
        <BButtons>
            <For each=button_color_options key=|bcolor| bcolor.clone() let:bcolor>
                <BButton color=bcolor.clone()>{String::from(bcolor)}</BButton>
            </For>

            <BButton color=button_color is_fullwidth=true on:click=button_on_click>
                "Click me to change my color  ("
                {move || String::from(button_color.get())}
                ")"
            </BButton>
        </BButtons>
    }
}
