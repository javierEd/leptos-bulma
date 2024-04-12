use leptos::*;

#[component]
pub fn BTitle(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional)] is: Option<i8>,
) -> impl IntoView {
    let title_class = move || {
        let mut t_class = "title".to_owned();

        if !class.get().is_empty() {
            t_class.push_str(&format!(" {}", class.get()));
        }

        if let Some(is) = is {
            t_class.push_str(&format!(" is-{}", is));
        }

        t_class
    };

    view! { <div class=title_class>{children()}</div> }
}

#[component]
pub fn BSubtitle(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional)] is: Option<i8>,
) -> impl IntoView {
    let subtitle_class = move || {
        let mut s_class = "subtitle".to_owned();

        if !class.get().is_empty() {
            s_class.push_str(&format!(" {}", class.get()));
        }

        if let Some(is) = is {
            s_class.push_str(&format!(" is-{}", is));
        }

        s_class
    };

    view! { <div class=subtitle_class>{children()}</div> }
}
