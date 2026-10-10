use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SkeletonBlockProps {
    /// How many grey text lines to draw (Bulma's `skeleton-lines`); zero draws
    /// one solid block (`skeleton-block`) instead.
    #[prop_or(3)]
    pub lines: u32,
    /// What a screen reader hears in place of the lines.
    #[prop_or_else(|| AttrValue::from("Loading…"))]
    pub label: AttrValue,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
}

/// A Bulma 1.x skeleton placeholder for content on its way: grey lines in the
/// shape of text, or one block. It is `aria-busy` and carries a visually
/// hidden label, so a screen reader hears "Loading…" rather than nothing.
#[component(SkeletonBlock)]
pub fn skeleton_block(props: &SkeletonBlockProps) -> Html {
    let label = html! { <span class="is-sr-only">{ props.label.clone() }</span> };
    if props.lines == 0 {
        return html! {
            <div class={classes!("skeleton-block", props.classes.clone())} aria-busy="true" data-testid={props.testid.clone()}>
                { label }
            </div>
        };
    }
    html! {
        <div class={classes!("skeleton-lines", props.classes.clone())} aria-busy="true" data-testid={props.testid.clone()}>
            { label }
            { for (0..props.lines).map(|_| html! { <div></div> }) }
        </div>
    }
}
