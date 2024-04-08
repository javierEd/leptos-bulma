use leptos::*;
use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarDivider, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};

#[component]
pub fn BasicNavbar() -> impl IntoView {
    let burger_is_active = create_rw_signal(false);

    view! {
        <BNavbar class="is-primary has-shadow block">
            <BNavbarBrand>
                <BNavbarItem class="is-size-4" href="/">
                    "Leptos Bulma"
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
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
                        <BNavbarDivider/>
                        <BNavbarItem>"Item"</BNavbarItem>
                    </BNavbarItemDropdown>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>
    }
}
