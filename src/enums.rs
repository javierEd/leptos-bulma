#[derive(Clone, Eq, Hash, PartialEq)]
pub enum BColor {
    Custom(String),
    Danger,
    Default,
    Info,
    Link,
    Primary,
    Success,
    Text,
    Warning,
}

impl From<BColor> for String {
    fn from(value: BColor) -> Self {
        match value {
            BColor::Custom(custom) => custom,
            BColor::Danger => "danger".to_owned(),
            BColor::Default => "default".to_owned(),
            BColor::Info => "info".to_owned(),
            BColor::Link => "link".to_owned(),
            BColor::Primary => "primary".to_owned(),
            BColor::Success => "success".to_owned(),
            BColor::Text => "text".to_owned(),
            BColor::Warning => "warning".to_owned(),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum BSize {
    Default,
    Large,
    Medium,
    Normal,
    Small,
}

impl From<BSize> for String {
    fn from(value: BSize) -> Self {
        match value {
            BSize::Default => "default",
            BSize::Large => "large",
            BSize::Medium => "medium",
            BSize::Normal => "normal",
            BSize::Small => "small",
        }
        .to_owned()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum BState {
    Active,
    Default,
    Disabled,
    Focused,
    Hovered,
    Loading,
}

impl From<BState> for String {
    fn from(value: BState) -> Self {
        match value {
            BState::Active => "active",
            BState::Default => "default",
            BState::Disabled => "disabled",
            BState::Focused => "focused",
            BState::Hovered => "hovered",
            BState::Loading => "loading",
        }
        .to_owned()
    }
}
