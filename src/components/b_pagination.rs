use leptos::*;
use leptos_router::use_location;

#[component]
pub fn BPagination(
    #[prop(optional, into)] class: TextProp,
    #[prop(into)] count: MaybeSignal<i16>,
    #[prop(default = 3)] list_size: i16,
) -> impl IntoView {
    let location = use_location();

    let current_page = move || {
        location
            .query
            .with(|q| q.get("page").cloned().unwrap_or("1".to_owned()))
            .parse::<i16>()
            .unwrap_or(1)
    };

    let page = create_rw_signal(current_page());

    let href = move |p: i16| format!("{}?page={}", location.pathname.get(), p);

    let previous_href = move || href(if page.get() > 1 { page.get() - 1 } else { 1 });
    let previous_class = move || {
        format!(
            "pagination-previous {}",
            if page.get() <= 1 { "is-disabled" } else { "" }
        )
    };

    let next_href = move || {
        href(if page.get() < count.get() {
            page.get() + 1
        } else {
            count.get()
        })
    };
    let next_class = move || {
        format!(
            "pagination-next {}",
            if count.get() <= page.get() {
                "is-disabled"
            } else {
                ""
            }
        )
    };

    let first_page_class = move || {
        format!(
            "pagination-link {}",
            if page.get() == 1 { "is-current" } else { "" }
        )
    };

    let last_page_class = move || {
        format!(
            "pagination-link {}",
            if page.get() == count.get() {
                "is-current"
            } else {
                ""
            },
        )
    };

    let count_is_greater = move || count.get() > list_size + 2;

    let left_side_size = (list_size / 2) - (list_size + 1) % 2;

    let right_side_size = list_size / 2;

    let pages_range = move || {
        let (start, end) = if count_is_greater() {
            let pg = page.get();
            let cnt = count.get();

            (
                if pg > cnt - list_size {
                    cnt - list_size
                } else if pg > left_side_size + 1 {
                    pg - left_side_size
                } else {
                    2
                },
                if pg <= list_size - right_side_size {
                    list_size + 2
                } else if pg < cnt - right_side_size {
                    pg + right_side_size + 1
                } else {
                    cnt
                },
            )
        } else {
            (2, count.get())
        };

        start..end
    };

    let show_first_ellipsis = move || count_is_greater() && page.get() > left_side_size + 2;

    let show_last_ellipsis =
        move || count_is_greater() && page.get() < count.get() - right_side_size - 1;

    create_effect(move |_| {
        page.set(current_page());
    });

    view! {
        <nav class=format!("pagination {}", class.get())>
            <a href=previous_href class=previous_class>
                "Previous"
            </a>
            <a href=next_href class=next_class>
                "Next"
            </a>

            <ul class="pagination-list">
                <li>
                    <a href=href(1) class=first_page_class>
                        "1"
                    </a>
                </li>

                <Show when=show_first_ellipsis>
                    <li>
                        <span class="pagination-ellipsis">"…"</span>
                    </li>
                </Show>

                <For each=pages_range key=|p| { *p } let:data>
                    <li>
                        <a
                            href=move || href(data)
                            class=move || {
                                format!("pagination-link {}", if page.get() == data { "is-current" } else { "" })
                            }
                        >

                            {data}
                        </a>
                    </li>
                </For>

                <Show when=show_last_ellipsis>
                    <li>
                        <span class="pagination-ellipsis">"…"</span>
                    </li>
                </Show>

                <Show when=move || { count.get() > 1 }>
                    <li>
                        <a href=move || href(count.get()) class=last_page_class>

                            {count.get()}
                        </a>
                    </li>
                </Show>
            </ul>
        </nav>
    }
}
