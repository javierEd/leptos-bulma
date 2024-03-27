use leptos::*;

use crate::EventFn;

#[component]
pub fn BInput(
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(optional, into)] on_input: Option<EventFn>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let mut b_input = view! {
        <input
            class=format!("input {}", class.unwrap_or_default())
            type=input_type
            id=id
            name=name
            placeholder=placeholder
            value=value
        />
    }
    .optional_event(ev::change, on_change.map(EventFn::into_inner))
    .optional_event(ev::input, on_input.map(EventFn::into_inner));

    for (attr_name, attr_value) in attributes {
        b_input = b_input.attr(attr_name, attr_value);
    }

    b_input
}
