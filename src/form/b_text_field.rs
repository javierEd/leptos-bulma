use leptos::*;

use super::{BControl, BField, BHelp, BLabel};

use crate::EventFn;

#[allow(unused_variables)]
#[component]
pub fn BTextField(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] error: MaybeSignal<Option<String>>,
    #[prop(optional, into)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(optional, into)] on_input: Option<EventFn>,
    #[prop(optional, into)] addon_left: Option<ViewFn>,
    #[prop(optional, into)] addon_right: Option<ViewFn>,
) -> impl IntoView {
    let error_text = create_rw_signal(None);

    create_effect(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let addon_left_view =
        addon_left.map(|addon_left| view! { <BControl>{addon_left.run()}</BControl> });

    let addon_right_view =
        addon_right.map(|addon_right| view! { <BControl>{addon_right.run()}</BControl> });

    let field_class = if addon_left_view.is_some() || addon_right_view.is_some() {
        "has-addons"
    } else {
        ""
    };

    let has_error = move || error_text.get().is_some();

    let input_class = move || {
        if has_error() {
            "input is-danger"
        } else {
            "input"
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

            <BField class=field_class>
                {addon_left_view} <BControl class="is-expanded">{input_view}</BControl> {addon_right_view}
            </BField>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
