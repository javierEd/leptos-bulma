use leptos::*;
use leptos_bulma::elements::BIcon;
use leptos_bulma::icons::*;

#[component]
pub fn IconPackages() -> impl IntoView {
    view! {
        "Ant Design Icons v4.3: "
        <BIcon icon=icondata_ai::AiBugFilled/>
        <br/>
        "BoxIcons v2.1: "
        <BIcon icon=icondata_bi::BiBugSolid/>
        <br/>
        "Bootstrap Icons v1.11: "
        <BIcon icon=icondata_bs::BsBugFill/>
        <br/>
        "css.gg v2.1: "
        <BIcon icon=icondata_cg::CgDebug/>
        <br/>
        "Charm v0.18: "
        <BIcon icon=icondata_ch::ChBug/>
        <br/>
        "FontAwesome v6.4: "
        <BIcon icon=icondata_fa::FaBugSolid/>
        <br/>
        "Feather v4.29: "
        <BIcon icon=icondata_fi::FiFeather/>
        <br/>
        "IcoMoon Free: "
        <BIcon icon=icondata_im::ImBug/>
        <br/>
        "Ionicons v7.1: "
        <BIcon icon=icondata_io::IoBug/>
        <br/>
        "Lucide v0.265: "
        <BIcon icon=icondata_lu::LuBug/>
        <br/>
        "Github Octicons v19.7: "
        <BIcon icon=icondata_oc::OcBugLg/>
        <br/>
        "Remix Icons v3.5: "
        <BIcon icon=icondata_ri::RiBugDevelopmentFill/>
        <br/>
        "Simple Icons v9.14: "
        <BIcon icon=icondata_si::SiOpenbugbounty/>
        <br/>
        "Tabler Icons v2.34: "
        <BIcon icon=icondata_tb::TbBug/>
        <br/>
        "Typicons v2.1: "
        <BIcon icon=icondata_ti::TiWorld/>
        <br/>
        "VS Code Icons v0.0: "
        <BIcon icon=icondata_vs::VsBug/>
        <br/>
        "Weather Icons v2.0: "
        <BIcon icon=icondata_wi::WiTornado/>
        <br/>
    }
}
