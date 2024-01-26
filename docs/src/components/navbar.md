# Navbar

Example:

```rust
use leptos::*;
use leptos_bulma::components::{
  BNavbar, BNavbarEnd, BNavbarBrand, BNavbarBurger, BNavbarItem, BNavbarMenu, BNavbarStart
};

#[component]
pub fn MyNavbar() -> impl IntoView {
    let (burger_is_active, set_burger_is_active) = create_signal(false);

    view! {
        <BNavbar class="is-primary">
            <BNavbarBrand>
                <BNavbarItem class="is-size-4" href="/">
                    "Leptos Bulma"
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active set_is_active=set_burger_is_active />
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem>"Item"</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem>"Item"</BNavbarItem>

                    <BNavbarItemDropdown
                        dropdown_class="is-right"
                        trigger=|| view! { <span class="has-text-weight-bold">"Dropdown item"</span> }
                    >
                        <BNavbarItem>"Item"</BNavbarItem>
                        <BNavbarDivider />
                        <BNavbarItem>"Item"</BNavbarItem>
                    </BNavbarItemDropdown>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>
    }
}
```

See it in action:

<iframe src="https://codesandbox.io/p/devbox/leptos-bulma-components-2252hl?file=%2Fsrc%2Fmain.rs&embed=1"
     title="Leptos Bulma Navbar"
     sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
   ></iframe>
