use leptos::*;

use crate::enums::{BColor, BSize};

#[component]
pub fn BProgress(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: MaybeSignal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: MaybeSignal<BSize>,
    #[prop(into)] max: MaybeSignal<usize>,
    #[prop(optional, into)] value: MaybeProp<usize>,
) -> impl IntoView {
    let mut class_list = "progress".to_owned();

    color.get().add_to_class_list(&mut class_list);

    size.get().add_to_class_list(&mut class_list);

    if !class.get().is_empty() {
        class_list += &format!(" {}", class.get());
    }

    view! {
        <progress class=class_list max=max value=value>
            {children.map(|c| c())}
        </progress>
    }
}
