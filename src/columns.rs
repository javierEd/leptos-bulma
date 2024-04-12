use leptos::*;

#[component]
pub fn BColumns(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("columns {}", class.get())>{children()}</div> }
}

#[component]
pub fn BColumn(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] is: TextProp,
    #[prop(optional, into)] is_offset: TextProp,
) -> impl IntoView {
    let column_class = move || {
        let mut column_class = "column".to_owned();

        if !is.get().is_empty() {
            column_class.push_str(&format!(" is-{}", is.get()));
        }

        if !is_offset.get().is_empty() {
            column_class.push_str(&format!(" is-offset-{}", is_offset.get()));
        }

        column_class.push_str(&format!(" {}", class.get()));

        column_class
    };

    view! { <div class=column_class>{children()}</div> }
}
