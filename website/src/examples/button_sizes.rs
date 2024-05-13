use leptos::*;
use leptos_bulma::elements::{BButton, BButtons};
use leptos_bulma::enums::BSize;

fn button_size_options() -> [BSize; 5] {
    [
        BSize::Small,
        BSize::Normal,
        BSize::Default,
        BSize::Medium,
        BSize::Large,
    ]
}

#[component]
pub fn ButtonSizes() -> impl IntoView {
    let button_size = create_rw_signal(*button_size_options().first().unwrap());

    let button_on_click = move |_| {
        let mut size_index = button_size_options()
            .iter()
            .position(|value| *value == button_size.get())
            .unwrap()
            + 1;

        if size_index >= button_size_options().len() {
            size_index = 0;
        }

        button_size.set(*button_size_options().get(size_index).unwrap());
    };

    view! {
        <BButtons>
            <For each=button_size_options key=|bsize| *bsize let:bsize>
                <BButton size=bsize>{String::from(bsize)}</BButton>
            </For>

            <BButton size=button_size is_fullwidth=true on:click=button_on_click>
                "Click me to change my size ("
                {move || String::from(button_size.get())}
                ")"
            </BButton>
        </BButtons>
    }
}
