use leptos::*;

use crate::EventFn;

#[allow(unused_variables)]
#[component]
pub fn BCheckbox(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] id: Option<&'static str>,
    label: &'static str,
    #[prop(optional, into)] name: Option<&'static str>,
    #[prop(default = "true".into(), into)] value: MaybeSignal<String>,
    #[prop(optional, into)] is_checked: MaybeSignal<bool>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let mut b_input =
        view! { <input node_ref=node_ref id=id name=name type="checkbox" value=value checked=is_checked/> }
            .optional_event(ev::change, on_change.map(EventFn::into_inner));

    for (attr_name, attr_value) in attributes {
        b_input = b_input.attr(attr_name, attr_value);
    }

    view! { <label class=format!("checkbox {}", class.get())>{b_input} " " {label}</label> }
}
