use std::hash::Hash;

use leptos::*;

#[component]
pub fn BTable(#[prop(optional, into)] class: TextProp, children: Children) -> impl IntoView {
    view! { <table class=format!("table {}", class.get())>{children()}</table> }
}

#[component]
pub fn BTbody<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_row: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static,
{
    view! {
        <tbody class=class>
            <For each=each_row key=key let:data>
                <tr>{children(data)}</tr>
            </For>
        </tbody>
    }
}

#[component]
pub fn BTfoot<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_cell: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static,
{
    view! {
        <tfoot class=class>
            <tr>
                <For each=each_cell key=key let:data>

                    <th>{children(data)}</th>
                </For>
            </tr>
        </tfoot>
    }
}

#[component]
pub fn BThead<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_cell: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T>,
    EF: Fn(T) -> N + 'static,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static,
{
    view! {
        <thead class=class>
            <tr>
                <For each=each_cell key=key let:data>

                    <th>{children(data)}</th>
                </For>
            </tr>
        </thead>
    }
}
