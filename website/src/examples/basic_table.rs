use leptos::*;
use leptos_bulma::elements::{BTable, BTbody, BTfoot, BThead};

#[component]
pub fn BasicTable() -> impl IntoView {
    let cells = move || [
        ("id", "ID".into_view()),
        ("fn", "First name".into_view()),
        ("ln", "Last name".into_view()),
        (
            "ln",
            view! { <abbr title="Average">"Avg"</abbr> }.into_view(),
        ),
    ];

    view! {
        <BTable class="is-fullwidth is-hoverable">
            <BThead cells=cells/>
            <BTfoot cells=cells/>
            <BTbody rows=[
                (
                    "1",
                    view! {
                        <th>"1"</th>
                        <td>"Foobar"</td>
                        <td>"Baz"</td>
                        <td>"9.5"</td>
                    },
                ),
                (
                    "2",
                    view! {
                        <th>"2"</th>
                        <td>"Fulano"</td>
                        <td>"Mengano"</td>
                        <td>"9.4"</td>
                    },
                ),
                (
                    "2",
                    view! {
                        <th>"2"</th>
                        <td>"Pepito"</td>
                        <td>"Zutano"</td>
                        <td>"9.3"</td>
                    },
                ),
            ]/>
        </BTable>
    }
}
