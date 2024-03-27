use leptos::*;

use super::{BControl, BField, BHelp, BLabel};

#[component]
pub fn BSelectField(
    #[prop(optional, into)] error: MaybeSignal<Option<String>>,
    #[prop(optional, into)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] options: MaybeSignal<Option<Vec<(String, String)>>>,
    #[prop(optional, into)] value: MaybeSignal<Option<String>>,
) -> impl IntoView {
    let error_text = create_rw_signal(None);

    create_effect(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let select_class = move || {
        if has_error() {
            "select is-danger"
        } else {
            "select"
        }
    };

    let options_view = move || {
        let options = options.clone().get().unwrap_or_default();
        let mut options_view = vec![];

        for option in options {
            let selected = if Some(option.1.clone()) == value.get() {
                Some("selected")
            } else {
                None
            };

            options_view.push(
                view! {
                    <option value=option.1.clone() selected=selected>
                        {option.0.clone()}
                    </option>
                }
                .into_view(),
            );
        }
    };

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BControl class="is-expanded">
                <div class=select_class>
                    <select id=id name=name>
                        {options_view}
                    </select>
                </div>
            </BControl>

            <Show when=has_error>
                <BHelp class="is-danger">{error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
