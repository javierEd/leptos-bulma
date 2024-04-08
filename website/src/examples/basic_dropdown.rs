use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem};

#[component]
pub fn BasicDropdown() -> impl IntoView {
    view! {
        <BDropdown class="block" trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }>
            <BDropdownItem>"Item"</BDropdownItem>
            <BDropdownDivider/>
            <BDropdownItem>"Item"</BDropdownItem>
        </BDropdown>
    }
}
