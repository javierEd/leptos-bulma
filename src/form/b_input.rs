use leptos::*;

#[component]
pub fn BInput(
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeSignal<String>,
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
    };

    for (attr_name, attr_value) in attributes {
        b_input = b_input.attr(attr_name, attr_value);
    }

    b_input
}
