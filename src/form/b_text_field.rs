use leptos::*;

use super::{BControl, BField, BHelp, BLabel};

#[component]
pub fn BTextField(
    #[prop(optional, into)] error: Option<MaybeSignal<Option<String>>>,
    #[prop(optional, into)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
) -> impl IntoView {
    let error_text = create_rw_signal(None);

    create_effect(move |_| {
        error_text.set(
            error
                .clone()
                .and_then(|e| e.get())
                .map(|e| e.trim().to_owned()),
        );
    });

    let has_error = move || error_text.get().is_some();

    let input_class = move || {
        if has_error() {
            "input is-danger"
        } else {
            "input"
        }
    };

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BControl class="is-expanded">
                <input class=input_class id=id type=input_type name=name placeholder=placeholder value=value/>
            </BControl>

            <Show when=has_error>
                <BHelp class="is-danger">{error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
