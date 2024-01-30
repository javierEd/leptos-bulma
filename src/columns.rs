use leptos::*;

#[component]
pub fn BColumns(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("columns {}", class)>{children()}</div>
    }
}

#[component]
pub fn BColumn(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] is: Option<&'static str>,
    #[prop(optional)] is_offset: Option<&'static str>,
) -> impl IntoView {
    let column_class = move || {
        let mut column_class = "column".to_owned();

        if let Some(is) = is {
            column_class.push_str(&format!(" is-{is}"));
        }

        if let Some(is_offset) = is_offset {
            column_class.push_str(&format!(" is-offset-{is_offset}"));
        }

        if let Some(class) = class {
            column_class.push_str(&format!(" {class}"));
        }

        column_class
    };

    view! {
        <div class=column_class>{children()}</div>
    }
}
