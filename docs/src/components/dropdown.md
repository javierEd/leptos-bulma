# Dropdown

Example:

```rust
use leptos::*;
use leptos_bulma::components::{BDropdown, BDropdownDivider, BDropdownItem};

#[component]
pub fn MyDropdown() -> impl IntoView {
    view! {
        <BDropdown
            class="m-4"
            trigger=|| view! { <span class="has-text-weight-bold">"Click here ▼"</span> }
        >
            <BDropdownItem>"Item"</BDropdownItem>
            <BDropdownDivider />
            <BDropdownItem>"Item"</BDropdownItem>
        </BDropdown>
    }
}
```

See it in action:

<iframe src="https://codesandbox.io/p/devbox/leptos-bulma-dropdown-s72jz6?file=%2Fsrc%2Fmain.rs&embed=1"
     title="Leptos Bulma Dropdown"
     sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
   ></iframe>
