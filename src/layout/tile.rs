use derive_more::Display;
use std::string::ToString;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// The context modifier to use for this tile element, else none.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
#[function_component(Tile)]
pub fn tile(props: &TileProps) -> Html {
    let ctx = props.ctx.as_ref().map(|ctx| ctx.to_string());
    let size = props.size.as_ref().map(|size| size.to_string());
    let class = classes!("tile", props.classes.clone(), ctx, props.vertical.then_some("is-vertical"), size);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

/// Tile context modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{}")]
pub enum TileCtx {
    #[display("ancestor")]
    Ancestor,
    #[display("parent")]
    Parent,
    #[display("child")]
    Child,
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{}")]
pub enum TileSize {
    #[display("1")]
    One,
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("10")]
    Ten,
    #[display("11")]
    Eleven,
    #[display("12")]
    Twelve,
}
