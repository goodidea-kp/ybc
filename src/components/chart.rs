//! Pie and donut charts drawn as inline SVG and styled with Bulma tokens.
//!
//! ```rust,ignore
//! let segments = vec![
//!     ybc::PieSegment::new("rust", "Rust", 61.0).with_href("/lang/rust"),
//!     ybc::PieSegment::new("go", "Go", 24.0).with_href("/lang/go"),
//!     ybc::PieSegment::new("ts", "TypeScript", 15.0).with_href("/lang/ts"),
//! ];
//! html! {
//!     <ybc::PieChart title="Lines of code by language" {segments} follow_href={false} on_select={go_to_language} />
//! }
//! ```
use std::f64::consts::TAU;

use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

const VIEWBOX_SIZE: f64 = 200.0;
const CENTER: f64 = VIEWBOX_SIZE / 2.0;
const OUTER_RADIUS: f64 = 88.0;
/// How far (in viewBox units) the hovered or focused segment grows outward.
const ACTIVE_LIFT: f64 = 4.0;
/// The largest hole ratio that still leaves a readable ring.
const MAX_HOLE: f32 = 0.85;
/// Below this ratio the centre is too small for the readout, which moves beside the chart.
const MIN_HOLE_FOR_CENTER_READOUT: f32 = 0.45;
const FULL_CIRCLE_EPSILON: f64 = 1e-6;
const INACTIVE_OPACITY: &str = "0.45";
/// The page surface, used for the 2px gap between segments. Falls back to white outside Bulma 1.x.
const SURFACE_COLOR: &str = "var(--bulma-scheme-main, #ffffff)";
const EMPTY_RING_COLOR: &str = "var(--bulma-border, #dbdbdb)";
const OTHER_COLOR: &str = "var(--ybc-chart-other, #8a8a8a)";

/// Categorical palette, assigned to segments in order. Each slot is a CSS custom property
/// (`--ybc-chart-1` .. `--ybc-chart-8`) with a light-surface fallback validated for
/// colour-vision deficiency; set the properties in your stylesheet to re-theme or to supply
/// dark-mode steps.
const PALETTE: [&str; 8] = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300", "#4a3aa7", "#e34948"];

/// One slice of a [`PieChart`].
#[derive(Clone, Debug, PartialEq)]
pub struct PieSegment {
    /// Stable identity of the segment, handed back by `on_select`. Also the render key.
    pub key: AttrValue,
    /// Human-readable name shown in the legend and the readout.
    pub label: AttrValue,
    /// The magnitude. Segments with a zero, negative or NaN value are not drawn.
    pub value: f64,
    /// Where the segment leads. Rendered as a real link, so open-in-new-tab and right-click work.
    pub href: Option<AttrValue>,
    /// Any CSS colour. Overrides the palette slot; use it when a segment has a brand colour.
    pub color: Option<AttrValue>,
}

impl PieSegment {
    pub fn new(key: impl Into<AttrValue>, label: impl Into<AttrValue>, value: f64) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            value,
            href: None,
            color: None,
        }
    }

    #[must_use]
    pub fn with_href(mut self, href: impl Into<AttrValue>) -> Self {
        self.href = Some(href.into());
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: impl Into<AttrValue>) -> Self {
        self.color = Some(color.into());
        self
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PieChartProps {
    /// The data, in the order the segments are drawn clockwise from 12 o'clock.
    ///
    /// Keep the order stable between renders so each segment keeps its colour.
    pub segments: Vec<PieSegment>,
    /// The chart's name. Shown as a caption and used as the accessible name.
    #[prop_or_default]
    pub title: AttrValue,
    /// Caption under the total in the centre readout.
    #[prop_or_else(|| AttrValue::from("Total"))]
    pub total_label: AttrValue,
    /// Radius of the hole as a fraction of the outer radius. `0.0` draws a pie; the default
    /// draws a donut whose centre holds the total.
    #[prop_or(0.6)]
    pub hole: f32,
    /// Segments beyond this count are folded into a single "Other" segment, keeping the chart
    /// readable. Capped at the palette size of 8.
    #[prop_or(6)]
    pub max_segments: usize,
    /// Label of the folded segment.
    #[prop_or_else(|| AttrValue::from("Other"))]
    pub other_label: AttrValue,
    /// Fired when a segment or its legend entry is clicked or activated from the keyboard.
    /// Receives the segment, so the handler can route to a detail page.
    #[prop_or_default]
    pub on_select: Callback<PieSegment>,
    /// Whether a plain click on a segment with an `href` lets the browser follow it.
    ///
    /// Set to `false` in a single-page app and navigate with `yew-router` from `on_select`
    /// instead; modified clicks (Ctrl, Cmd, middle button) always fall through to the browser.
    #[prop_or(true)]
    pub follow_href: bool,
    /// Custom number formatting for the readout and legend. Defaults to thousands separators
    /// and at most one decimal.
    #[prop_or_default]
    pub format_value: Option<Callback<f64, String>>,
    /// CSS width of the chart itself, e.g. `"16rem"` or `"100%"`.
    #[prop_or_else(|| AttrValue::from("16rem"))]
    pub size: AttrValue,
    /// Show the legend. It doubles as the table view, so hide it only when the values are
    /// listed elsewhere on the page.
    #[prop_or(true)]
    pub legend: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// A segment after filtering and folding, with its share of the whole and its colour.
#[derive(Clone, Debug, PartialEq)]
struct Slice {
    segment: PieSegment,
    start: f64,
    end: f64,
    fill: String,
}

impl Slice {
    fn fraction(&self) -> f64 {
        self.end - self.start
    }
}

/// A part-to-whole chart: a donut by default, a pie with `hole={0.0}`.
///
/// Every segment is a hit target with hover and keyboard focus, a native link when it has an
/// `href`, and a source of `on_select` events for router navigation. The legend mirrors the
/// segments with values and shares, so nothing is encoded by colour alone.
#[component(PieChart)]
pub fn pie_chart(props: &PieChartProps) -> Html {
    let active = use_state(|| None::<usize>);
    let slices = build_slices(props);
    let total: f64 = slices.iter().map(|slice| slice.segment.value).sum();
    let hole = props.hole.clamp(0.0, MAX_HOLE);
    let inner_radius = OUTER_RADIUS * f64::from(hole);
    let readout_in_center = hole >= MIN_HOLE_FOR_CENTER_READOUT;
    let format = value_formatter(props);
    let active_index = *active;

    let set_active = {
        let active = active.clone();
        Callback::from(move |index: Option<usize>| active.set(index))
    };

    let drawn = slices.iter().enumerate().map(|(index, slice)| {
        let view = SliceView {
            slice: slice.clone(),
            index,
            inner_radius,
            is_active: active_index == Some(index),
            is_dimmed: active_index.is_some() && active_index != Some(index),
            aria_label: format!(
                "{}: {} ({})",
                slice.segment.label,
                format.emit(slice.segment.value),
                format_percent(slice.fraction())
            ),
        };
        render_slice(view, props, &set_active)
    });

    let readout = render_readout(&slices, active_index, total, props, &format);
    let chart_style = format!("position:relative;flex:0 0 auto;width:{};max-width:100%", props.size);
    let aria_label = if props.title.is_empty() {
        AttrValue::from("Pie chart")
    } else {
        props.title.clone()
    };

    html! {
        <figure id={props.id.clone()} class={classes!("ybc-pie", props.classes.clone())}>
            if !props.title.is_empty() {
                <figcaption class="has-text-weight-semibold mb-3">{props.title.clone()}</figcaption>
            }
            <div class="is-flex is-flex-wrap-wrap is-align-items-center" style="gap:1.5rem">
                <div style={chart_style}>
                    <svg viewBox={format!("0 0 {VIEWBOX_SIZE} {VIEWBOX_SIZE}")} role="group" aria-label={aria_label} style="display:block;width:100%;height:auto;overflow:visible">
                        if slices.is_empty() {
                            <path d={ring_path(OUTER_RADIUS, inner_radius)} fill={EMPTY_RING_COLOR} fill-rule="evenodd" />
                        } else {
                            { for drawn }
                        }
                    </svg>
                    if readout_in_center {
                        <div style="position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;text-align:center;padding:24%;pointer-events:none">
                            {readout.clone()}
                        </div>
                    }
                </div>
                <div style="flex:1 1 12rem;min-width:0">
                    if !readout_in_center {
                        <div class="mb-3">{readout}</div>
                    }
                    if props.legend && !slices.is_empty() {
                        {render_legend(&slices, active_index, props, &format, &set_active)}
                    }
                </div>
            </div>
        </figure>
    }
}

/// Drop unusable values, fold the tail into "Other", and assign shares and colours.
fn build_slices(props: &PieChartProps) -> Vec<Slice> {
    let mut segments: Vec<PieSegment> = props
        .segments
        .iter()
        .filter(|segment| segment.value.is_finite() && segment.value > 0.0)
        .cloned()
        .collect();
    let max_segments = props.max_segments.clamp(1, PALETTE.len());
    if segments.len() > max_segments {
        let folded: f64 = segments.drain(max_segments - 1..).map(|segment| segment.value).sum();
        segments.push(PieSegment::new("other", props.other_label.clone(), folded).with_color(OTHER_COLOR));
    }
    let total: f64 = segments.iter().map(|segment| segment.value).sum();
    let mut start = 0.0;
    segments
        .into_iter()
        .enumerate()
        .map(|(slot, segment)| {
            let end = start + segment.value / total;
            let fill = segment
                .color
                .as_ref()
                .map(|color| color.to_string())
                .unwrap_or_else(|| format!("var(--ybc-chart-{}, {})", slot + 1, PALETTE[slot]));
            let slice = Slice { segment, start, end, fill };
            start = end;
            slice
        })
        .collect()
}

fn value_formatter(props: &PieChartProps) -> Callback<f64, String> {
    props.format_value.clone().unwrap_or_else(|| Callback::from(format_number))
}

/// Everything needed to draw one segment.
struct SliceView {
    slice: Slice,
    index: usize,
    inner_radius: f64,
    is_active: bool,
    is_dimmed: bool,
    aria_label: String,
}

fn render_slice(view: SliceView, props: &PieChartProps, set_active: &Callback<Option<usize>>) -> Html {
    let outer_radius = if view.is_active { OUTER_RADIUS + ACTIVE_LIFT } else { OUTER_RADIUS };
    let path = slice_path(view.slice.start, view.slice.end, outer_radius, view.inner_radius);
    let opacity = if view.is_dimmed { INACTIVE_OPACITY } else { "1" };
    let has_href = view.slice.segment.href.is_some();
    let onclick = select_on_click(props, &view.slice.segment);
    let onkeydown = select_on_key(props, &view.slice.segment, has_href);
    let on_enter = set_active.reform(move |_: MouseEvent| Some(view.index));
    let on_focus = set_active.reform(move |_: FocusEvent| Some(view.index));
    let on_leave = set_active.reform(|_: MouseEvent| None);
    let on_blur = set_active.reform(|_: FocusEvent| None);
    let shape = html! {
        <>
            <title>{view.aria_label.clone()}</title>
            <path d={path} fill={view.slice.fill.clone()} stroke={SURFACE_COLOR} stroke-width="2" fill-rule="evenodd" {opacity} style="transition:opacity .15s" />
        </>
    };
    if let Some(href) = view.slice.segment.href.clone() {
        html! {
            <a key={view.slice.segment.key.to_string()} {href} aria-label={view.aria_label.clone()} style="cursor:pointer"
               {onclick} {onkeydown} onmouseenter={on_enter} onmouseleave={on_leave} onfocus={on_focus} onblur={on_blur}>
                {shape}
            </a>
        }
    } else {
        html! {
            <g key={view.slice.segment.key.to_string()} role="button" tabindex="0" aria-label={view.aria_label.clone()} style="cursor:pointer"
               {onclick} {onkeydown} onmouseenter={on_enter} onmouseleave={on_leave} onfocus={on_focus} onblur={on_blur}>
                {shape}
            </g>
        }
    }
}

fn select_on_click(props: &PieChartProps, segment: &PieSegment) -> Callback<MouseEvent> {
    let on_select = props.on_select.clone();
    let follow_href = props.follow_href;
    let segment = segment.clone();
    Callback::from(move |event: MouseEvent| {
        if is_modified_click(&event) {
            return;
        }
        if !follow_href {
            event.prevent_default();
        }
        on_select.emit(segment.clone());
    })
}

/// Space activates every segment; Enter only the ones that are not links, whose native
/// activation already fires `click`.
fn select_on_key(props: &PieChartProps, segment: &PieSegment, has_href: bool) -> Callback<KeyboardEvent> {
    let on_select = props.on_select.clone();
    let follow_href = props.follow_href;
    let segment = segment.clone();
    Callback::from(move |event: KeyboardEvent| {
        let key = event.key();
        let activates = key == " " || (key == "Enter" && (!has_href || !follow_href));
        if !activates {
            return;
        }
        event.prevent_default();
        on_select.emit(segment.clone());
    })
}

fn is_modified_click(event: &MouseEvent) -> bool {
    event.button() != 0 || event.ctrl_key() || event.meta_key() || event.shift_key() || event.alt_key()
}

fn render_readout(slices: &[Slice], active_index: Option<usize>, total: f64, props: &PieChartProps, format: &Callback<f64, String>) -> Html {
    let (value, label, share) = match active_index.and_then(|index| slices.get(index)) {
        Some(slice) => (
            format.emit(slice.segment.value),
            slice.segment.label.clone(),
            Some(format_percent(slice.fraction())),
        ),
        None if slices.is_empty() => (String::from("–"), AttrValue::from("No data"), None),
        None => (format.emit(total), props.total_label.clone(), None),
    };
    html! {
        <div style="display:flex;flex-direction:column;align-items:center;max-width:100%">
            <span class="is-size-4 has-text-weight-bold" style="line-height:1.1">{value}</span>
            <span class="is-size-7 has-text-grey" style="max-width:100%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{label}</span>
            if let Some(share) = share {
                <span class="is-size-7">{share}</span>
            }
        </div>
    }
}

fn render_legend(
    slices: &[Slice], active_index: Option<usize>, props: &PieChartProps, format: &Callback<f64, String>, set_active: &Callback<Option<usize>>,
) -> Html {
    let items = slices.iter().enumerate().map(|(index, slice)| {
        let is_active = active_index == Some(index);
        let style = format!(
            "display:flex;align-items:center;gap:.5rem;width:100%;border:0;padding:.25rem .5rem;border-radius:4px;cursor:pointer;color:inherit;font:inherit;text-align:left;text-decoration:none;background:{}",
            if is_active { "var(--bulma-background, #f5f5f5)" } else { "transparent" }
        );
        let swatch_style = format!("flex:none;width:.75rem;height:.75rem;border-radius:3px;background:{}", slice.fill);
        let onclick = select_on_click(props, &slice.segment);
        let on_enter = set_active.reform(move |_: MouseEvent| Some(index));
        let on_focus = set_active.reform(move |_: FocusEvent| Some(index));
        let on_leave = set_active.reform(|_: MouseEvent| None);
        let on_blur = set_active.reform(|_: FocusEvent| None);
        let content = html! {
            <>
                <span aria-hidden="true" style={swatch_style}></span>
                <span style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{slice.segment.label.clone()}</span>
                <span class="has-text-weight-semibold">{format.emit(slice.segment.value)}</span>
                <span class="is-size-7 has-text-grey" style="min-width:3.5ch;text-align:right">{format_percent(slice.fraction())}</span>
            </>
        };
        html! {
            <li key={slice.segment.key.to_string()}>
                if let Some(href) = slice.segment.href.clone() {
                    <a {href} {style} {onclick} onmouseenter={on_enter} onmouseleave={on_leave} onfocus={on_focus} onblur={on_blur}>{content}</a>
                } else {
                    <button type="button" {style} {onclick} onmouseenter={on_enter} onmouseleave={on_leave} onfocus={on_focus} onblur={on_blur}>{content}</button>
                }
            </li>
        }
    });
    html! {
        <ul aria-label="Legend" style="list-style:none;margin:0;padding:0">
            { for items }
        </ul>
    }
}

/// A point on the circle, with `fraction` measured clockwise from 12 o'clock.
fn point_at(radius: f64, fraction: f64) -> String {
    let angle = fraction * TAU - TAU / 4.0;
    format!("{:.3} {:.3}", CENTER + radius * angle.cos(), CENTER + radius * angle.sin())
}

/// SVG path of one segment; a ring segment when `inner` is positive, a wedge otherwise.
fn slice_path(start: f64, end: f64, outer: f64, inner: f64) -> String {
    if end - start >= 1.0 - FULL_CIRCLE_EPSILON {
        return ring_path(outer, inner);
    }
    let large_arc = u8::from(end - start > 0.5);
    let outer_start = point_at(outer, start);
    let outer_end = point_at(outer, end);
    if inner <= 0.0 {
        return format!("M{CENTER} {CENTER} L{outer_start} A{outer} {outer} 0 {large_arc} 1 {outer_end} Z");
    }
    let inner_start = point_at(inner, start);
    let inner_end = point_at(inner, end);
    format!("M{outer_start} A{outer} {outer} 0 {large_arc} 1 {outer_end} L{inner_end} A{inner} {inner} 0 {large_arc} 0 {inner_start} Z")
}

/// A full circle (or ring, with `fill-rule="evenodd"`) as two half arcs, since a single SVG
/// arc cannot describe 360 degrees.
fn ring_path(outer: f64, inner: f64) -> String {
    let mut path = full_circle(outer);
    if inner > 0.0 {
        path.push(' ');
        path.push_str(&full_circle(inner));
    }
    path
}

fn full_circle(radius: f64) -> String {
    let top = point_at(radius, 0.0);
    let bottom = point_at(radius, 0.5);
    format!("M{top} A{radius} {radius} 0 1 1 {bottom} A{radius} {radius} 0 1 1 {top} Z")
}

fn format_percent(fraction: f64) -> String {
    let percent = fraction * 100.0;
    if percent < 1.0 { String::from("<1%") } else { format!("{percent:.0}%") }
}

/// Thousands separators, and at most one decimal.
fn format_number(value: f64) -> String {
    let text = if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    };
    let (sign, unsigned) = text.strip_prefix('-').map_or(("", text.as_str()), |rest| ("-", rest));
    let (integer, decimals) = unsigned
        .split_once('.')
        .map_or((unsigned, None), |(integer, decimals)| (integer, Some(decimals)));
    let digit_count = integer.len();
    let grouped: String = integer
        .chars()
        .enumerate()
        .flat_map(|(position, digit)| {
            let needs_separator = position > 0 && (digit_count - position) % 3 == 0;
            needs_separator.then_some(',').into_iter().chain(std::iter::once(digit))
        })
        .collect();
    match decimals {
        Some(decimals) => format!("{sign}{grouped}.{decimals}"),
        None => format!("{sign}{grouped}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_number_groups_thousands_and_trims_whole_values() {
        assert_eq!(format_number(1234567.0), "1,234,567");
        assert_eq!(format_number(999.0), "999");
        assert_eq!(format_number(1234.56), "1,234.6");
        assert_eq!(format_number(-1000.0), "-1,000");
    }

    #[test]
    fn format_percent_floors_tiny_shares() {
        assert_eq!(format_percent(0.004), "<1%");
        assert_eq!(format_percent(0.256), "26%");
    }

    fn props_with(segments: Vec<PieSegment>, max_segments: usize) -> PieChartProps {
        PieChartProps {
            segments,
            title: AttrValue::default(),
            total_label: AttrValue::from("Total"),
            hole: 0.6,
            max_segments,
            other_label: AttrValue::from("Other"),
            on_select: Callback::default(),
            follow_href: true,
            format_value: None,
            size: AttrValue::from("16rem"),
            legend: true,
            classes: Classes::default(),
            id: None,
        }
    }

    #[test]
    fn build_slices_drops_unusable_values_and_folds_the_tail() {
        let segments = (1..=8)
            .map(|index| PieSegment::new(index.to_string(), index.to_string(), f64::from(index)))
            .chain([PieSegment::new("zero", "zero", 0.0), PieSegment::new("nan", "nan", f64::NAN)])
            .collect();
        let slices = build_slices(&props_with(segments, 4));
        assert_eq!(slices.len(), 4);
        assert_eq!(slices[3].segment.label, "Other");
        assert_eq!(slices[3].segment.value, 4.0 + 5.0 + 6.0 + 7.0 + 8.0);
        assert!((slices.last().map(|slice| slice.end).unwrap_or_default() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn a_single_segment_draws_a_full_ring() {
        let path = slice_path(0.0, 1.0, OUTER_RADIUS, 40.0);
        assert!(path.starts_with("M100.000 12.000 A88 88 0 1 1"));
        assert!(path.contains("A40 40 0 1 1"));
    }
}
