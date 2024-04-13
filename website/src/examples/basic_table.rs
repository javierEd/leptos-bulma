use leptos::*;
use leptos_bulma::elements::{BTable, BTbody, BTfoot, BThead};

#[component]
pub fn BasicTable() -> impl IntoView {
    let cells = move || {
        [
            ("id", "ID".into_view()),
            ("fn", "First name".into_view()),
            ("ln", "Last name".into_view()),
            (
                "ln",
                view! { <abbr title="Average">"Avg"</abbr> }.into_view(),
            ),
        ]
    };

    view! {
        <BTable class="is-fullwidth is-hoverable">
            <BThead each_cell=cells key=|cell| { cell.0 } let:data>
                {data.1}
            </BThead>
            <BTfoot each_cell=cells key=|cell| { cell.0 } let:data>
                {data.1}
            </BTfoot>
            <BTbody
                each_row=move || [
                    (1, "Foobar", "Baz", 9.5),
                    (2, "Fulano", "Mengano", 9.4),
                    (3, "Pepito", "Zutano", 9.3),
                ]

                key=|row| { row.0 }
                let:data
            >
                <th>{data.0}</th>
                <td>{data.1}</td>
                <td>{data.2}</td>
                <td>{data.3}</td>
            </BTbody>
        </BTable>
    }
}
