use leptos::*;

use crate::enums::BColor;

#[component]
pub fn BNotification(
    children: ChildrenFn,
    #[prop(optional, into)] class: TextProp,

    #[prop(default = BColor::Default.into(), into)] color: MaybeSignal<BColor>,
    #[prop(optional, into)] is_light: MaybeSignal<bool>,
    #[prop(into)] is_active: RwSignal<bool>,
) -> impl IntoView {
    let class_list = move || {
        let mut class_list = "notification".to_owned();

        color.get().add_to_class_list(&mut class_list);

        if is_light.get() {
            class_list += " is-light";
        }

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
    };

    let class_list = store_value(class_list);

    view! {
        <Show when=move || { is_active.get() }>
            <div class=class_list.with_value(|class_list| class_list())>
                <button class="delete" on:click=move |_| { is_active.set(false) }></button>
                {children()}
            </div>
        </Show>
    }
}
