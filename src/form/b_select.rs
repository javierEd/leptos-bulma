use leptos::*;

#[component]
pub fn BSelect(
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] options: MaybeSignal<Option<Vec<(String, String)>>>,
    #[prop(optional, into)] value: MaybeSignal<Option<String>>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
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

    let mut b_select = view! {
        <div class="select">
            <select id=id name=name>
                {options_view}
            </select>
        </div>
    };

    for (attr_name, attr_value) in attributes {
        b_select = b_select.attr(attr_name, attr_value);
    }

    b_select
}
