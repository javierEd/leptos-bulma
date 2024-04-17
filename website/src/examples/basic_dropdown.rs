use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem};

#[component]
pub fn BasicDropdown() -> impl IntoView {
    let show_alert = move |_| {
        let _ = window().alert_with_message("Item clicked");
    };

    view! {
        <BDropdown class="block" trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }>
            <BDropdownItem on:click=show_alert>"Item"</BDropdownItem>
            <BDropdownDivider/>
            <BDropdownItem on:click=show_alert>"Item"</BDropdownItem>
        </BDropdown>

        <BDropdown
            class="block ml-4"
            is_hoverable=true
            trigger=|| view! { <span class="has-text-weight-bold">"Hover here ▼"</span> }
        >
            <BDropdownItem on:click=show_alert>"Item"</BDropdownItem>
            <BDropdownDivider/>
            <BDropdownItem on:click=show_alert>"Item"</BDropdownItem>
        </BDropdown>
    }
}
