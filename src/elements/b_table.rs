use leptos::*;

#[component]
pub fn BTable(#[prop(optional, into)] class: TextProp, children: Children) -> impl IntoView {
    view! { <table class=format!("table {}", class.get())>{children()}</table> }
}

#[component]
pub fn BTbody<I, IV>(
    #[prop(optional, into)] class: TextProp,
    #[prop(into)] rows: MaybeSignal<I>,
) -> impl IntoView
where
    I: IntoIterator<Item = (&'static str, IV)> + 'static + Clone,
    IV: IntoView + 'static + Clone,
{
    view! {
        <tbody class=class>
            <For each=move || { rows.get() } key=|(k, _)| { *k } let:row>
                <tr>{row.1}</tr>
            </For>
        </tbody>
    }
}

#[component]
pub fn BTfoot<IF, I, IV>(#[prop(optional, into)] class: TextProp, cells: IF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = (&'static str, IV)>,
    IV: IntoView + 'static + Clone,
{
    view! {
        <tfoot class=class>
            <tr>
                <For each=cells key=|(k, _)| { *k } let:cell>

                    <th>{cell.1}</th>
                </For>
            </tr>
        </tfoot>
    }
}

#[component]
pub fn BThead<IF, I, IV>(#[prop(optional, into)] class: TextProp, cells: IF) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = (&'static str, IV)>,
    IV: IntoView + 'static + Clone,
{
    view! {
        <thead class=class>
            <tr>
                <For each=cells key=|(k, _)| { *k } let:cell>

                    <th>{cell.1}</th>
                </For>
            </tr>
        </thead>
    }
}
