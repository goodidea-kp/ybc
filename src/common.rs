use derive_more::Display;
use std::borrow::Cow;
use yew::html::IntoPropValue;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum Alignment {
    #[display("is-left")]
    Left,
    #[display("is-centered")]
    Centered,
    #[display("is-right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum Size {
    #[display("is-small")]
    Small,
    #[display("is-normal")]
    Normal,
    #[display("is-medium")]
    Medium,
    #[display("is-large")]
    Large,
}

impl IntoPropValue<Cow<'static, str>> for Size {
    fn into_prop_value(self) -> Cow<'static, str> {
        Cow::from(self.to_string())
    }
}
