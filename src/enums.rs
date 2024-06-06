#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum BAlignment {
    Centered,
    Default,
    Right,
}

impl From<BAlignment> for String {
    fn from(value: BAlignment) -> Self {
        match value {
            BAlignment::Centered => "centered",
            BAlignment::Default => "default",
            BAlignment::Right => "right",
        }
        .to_owned()
    }
}

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

impl BColor {
    pub(crate) fn add_to_class_list(&self, class_list: &mut String) {
        if self != &BColor::Default {
            class_list.push_str(&format!(" is-{}", String::from(self)))
        };
    }
}

impl From<BColor> for String {
    fn from(value: BColor) -> Self {
        String::from(&value)
    }
}

impl From<&BColor> for String {
    fn from(value: &BColor) -> Self {
        match value {
            BColor::Custom(custom) => custom.clone(),
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

impl BSize {
    pub(crate) fn add_to_class_list(&self, class_list: &mut String) {
        if self != &BSize::Default {
            class_list.push_str(&format!(" is-{}", String::from(self)))
        };
    }
}

impl From<BSize> for String {
    fn from(value: BSize) -> Self {
        String::from(&value)
    }
}

impl From<&BSize> for String {
    fn from(value: &BSize) -> Self {
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
pub enum BBreadcrumbSeparator {
    Arrow,
    Bullet,
    Default,
    Dot,
    Succeeds,
}

impl From<BBreadcrumbSeparator> for String {
    fn from(value: BBreadcrumbSeparator) -> Self {
        match value {
            BBreadcrumbSeparator::Arrow => "arrow",
            BBreadcrumbSeparator::Bullet => "bullet",
            BBreadcrumbSeparator::Default => "default",
            BBreadcrumbSeparator::Dot => "dot",
            BBreadcrumbSeparator::Succeeds => "ducceeds",
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
