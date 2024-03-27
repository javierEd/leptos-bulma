use leptos::*;
use leptos_icons::Icon;

use super::{BControl, BField, BHelp, BLabel};

#[component]
pub fn BPasswordField(
    #[prop(optional, into)] error: MaybeSignal<Option<String>>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
) -> impl IntoView {
    let error_text = create_rw_signal(None);
    let is_visible = create_rw_signal(false);

    create_effect(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let input_class = move || {
        if has_error() {
            "input is-danger"
        } else {
            "input"
        }
    };

    let button_class = move || {
        if has_error() {
            "button is-danger is-outlined"
        } else {
            "button"
        }
    };

    let input_type = move || {
        if is_visible.get() {
            "text"
        } else {
            "password"
        }
    };

    let visibility_icon = move || {
        if is_visible.get() {
            view! { <Icon icon=icondata_fa::FaEyeSolid/> }
        } else {
            view! { <Icon icon=icondata_fa::FaEyeSlashSolid/> }
        }
    };

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BField class="has-addons">
                <BControl class="is-expanded">
                    <input class=input_class id=id type=input_type name=name placeholder=placeholder value=value/>
                </BControl>

                <BControl>
                    <a class=button_class on:click=move |_| { is_visible.update(|v| *v = !*v) }>
                        <span class="icon">
                            {visibility_icon}
                        </span>
                    </a>
                </BControl>
            </BField>

            <Show when=has_error>
                <BHelp class="is-danger">{error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
