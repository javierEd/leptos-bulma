use leptos::*;
use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarDivider, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};

#[component]
pub fn BasicNavbar() -> impl IntoView {
    let burger_is_active = create_rw_signal(false);

    let show_alert = move |_| {
        let _ = window().alert_with_message("Item clicked");
    };

    view! {
        <BNavbar class="is-primary has-shadow block">
            <BNavbarBrand>
                <BNavbarItem class="is-size-4" on:click=show_alert>
                    "Leptos Bulma"
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>

                    <BNavbarItemDropdown
                        dropdown_class="is-right"
                        trigger=|| view! { <span class="has-text-weight-bold">"Dropdown item"</span> }
                    >
                        <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                        <BNavbarDivider/>
                        <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                    </BNavbarItemDropdown>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>
    }
}
