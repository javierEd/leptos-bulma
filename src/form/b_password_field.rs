use leptos::*;

use crate::EventFn;

use super::{BControl, BField, BHelp, BLabel};

#[cfg(feature = "icondata-fa")]
#[component]
fn VisibilityIcon(is_visible: RwSignal<bool>) -> impl IntoView {
    use crate::elements::BIcon;

    let visibility_icon = create_rw_signal(icondata_fa::FaEyeSlashSolid);

    create_effect(move |_| {
        visibility_icon.set(if is_visible.get() {
            icondata_fa::FaEyeSolid
        } else {
            icondata_fa::FaEyeSlashSolid
        })
    });

    view! { <BIcon is_scaled=false icon=visibility_icon/> }
}

#[cfg(not(feature = "icondata-fa"))]
#[component]
fn VisibilityIcon(is_visible: RwSignal<bool>) -> impl IntoView {
    let text_decoration = create_rw_signal("line-through");

    create_effect(move |_| {
        text_decoration.set(if is_visible.get() {
            "none"
        } else {
            "line-through"
        })
    });

    view! { <span style:text-decoration=text_decoration>"👁"</span> }
}

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
                    <a class=button_class on:click=move |_| { is_visible.update(|value| *value = !*value) }>
                        <VisibilityIcon is_visible=is_visible/>
                    </a>
                </BControl>
            </BField>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
