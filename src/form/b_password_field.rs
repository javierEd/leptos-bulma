use leptos::*;
use leptos_icons::Icon;

use crate::EventFn;

use super::{BControl, BField, BHelp, BLabel};

#[allow(unused_variables)]
#[component]
pub fn BPasswordField(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] error: MaybeSignal<Option<String>>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(optional, into)] on_input: Option<EventFn>,
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

    let input_view = view! {
        <input node_ref=node_ref class=input_class id=id type=input_type name=name placeholder=placeholder value=value/>
    }
    .optional_event(ev::change, on_change.map(EventFn::into_inner))
    .optional_event(ev::input, on_input.map(EventFn::into_inner));

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BField class="has-addons">
                <BControl class="is-expanded">{input_view}</BControl>

                <BControl>
                    <a class=button_class on:click=move |_| { is_visible.update(|v| *v = !*v) }>
                        <span class="icon">{visibility_icon}</span>
                    </a>
                </BControl>
            </BField>

            <Show when=has_error>
                <BHelp class="is-danger">{error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
