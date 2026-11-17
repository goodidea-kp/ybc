//! A word-sized trend line drawn as inline SVG: no chart library, no script to load.
//!
//! ```rust,ignore
//! let rows_per_second = vec![48_000.0, 71_000.0, 63_200.0, 56_590.0];
//! let whole = Callback::from(|value: f64| ybc::format_number(value.round()));
//! html! {
//!     <ybc::Sparkline values={rows_per_second} aria_label="Copy rate" label="Copy rate" unit="rows/s"
//!                     unit_words="rows per second" span="last 4 min" span_words="last 4 minutes"
//!                     point_labels={vec!["3 min ago".into(), "2 min ago".into(), "1 min ago".into(), "now".into()]}
//!                     format_value={whole} />
//! }
//! ```
//!
//! It says what it draws without a hover: with a `label`, the label stands before the line and the
//! latest value and the range ("48k–71k rows/s, last 4 min") after it. Each point is there on
//! demand: the line is one tab stop; a hover, a tap or keyboard focus shows a point's time and
//! value, the arrow keys (and Home/End) move between points, Escape closes it, and the pointer may
//! rest on the text it shows (WCAG 1.4.13). A screen reader hears one image with a summary and,
//! as the arrows move, each point. Nothing moves by itself, so there is nothing to reduce.
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use super::chart::format_number;
use super::sparkline_parts::{
    Frame, INSET, Point, SummaryWords, compact_number, drawn_points, extremes, moved_by_key, nearest_index, point_text, range_text, stats,
    summary_text, with_unit,
};

const STROKE_WIDTH: &str = "1.5";
const LAST_POINT_RADIUS: &str = "2";
const EXTREME_RADIUS: &str = "2";
const EXTREME_STROKE_WIDTH: &str = "1.25";
const ACTIVE_RADIUS: &str = "3.5";
const EMPTY_DASHES: &str = "2 3";
const EMPTY_OPACITY: &str = "0.4";
/// The page surface, so an extreme's ring reads as hollow in both schemes.
const SURFACE_COLOR: &str = "var(--bulma-scheme-main, #ffffff)";
/// Between a point's time and its value on screen.
const SHOWN_SEPARATOR: &str = " \u{00b7} ";
/// Between a point's time and its value, read aloud.
const SPOKEN_SEPARATOR: &str = ": ";
/// Points in the left third anchor their tooltip left, in the right third right, else centred.
const ANCHOR_THIRDS: f64 = 3.0;

/// The look, in Bulma's scheme variables only, so it follows the light and dark schemes and forced
/// colours. No CSS comments: this text reaches every page (see the modal's). No transition or
/// animation, so reduced motion has nothing to stop.
const SPARKLINE_STYLE: &str = r#"
.ybc-sparkline { display: inline-flex; flex-wrap: wrap; align-items: center; column-gap: 0.4em; vertical-align: middle; }
.ybc-sparkline-plot { position: relative; display: inline-flex; }
.ybc-sparkline-line { display: block; overflow: visible; cursor: crosshair; touch-action: manipulation; }
.ybc-sparkline-line:focus { outline: none; }
.ybc-sparkline-line:focus-visible { outline: 2px solid var(--bulma-link-on-scheme, currentColor); outline-offset: 3px; border-radius: 2px; }
.ybc-sparkline-range { font-size: 0.85em; color: var(--bulma-text-weak, inherit); }
.ybc-sparkline-tip { position: absolute; bottom: 100%; z-index: 30; padding-bottom: 0.3rem; white-space: nowrap; }
.ybc-sparkline-tip > span { display: block; padding: 0.2rem 0.45rem; border: 1px solid transparent; border-radius: var(--bulma-radius, 4px);
    background: var(--bulma-scheme-invert, #14161a); color: var(--bulma-scheme-main, #ffffff); font-size: 0.8rem; line-height: 1.3; }
@media (forced-colors: active) {
    .ybc-sparkline-line polyline, .ybc-sparkline-line circle { stroke: CanvasText; }
    .ybc-sparkline-line circle.is-filled { fill: CanvasText; }
    .ybc-sparkline-tip > span { border-color: CanvasText; }
}
"#;

/// The figures a [`Sparkline`] summarises for a screen reader, over the drawn values only.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SparklineStats {
    pub min: f64,
    pub max: f64,
    /// The latest value: the last one drawn.
    pub last: f64,
    /// How many values were drawn (NaN and infinite values are left out).
    pub count: usize,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SparklineProps {
    /// The series, oldest first. NaN and infinite values are left out.
    pub values: Vec<f64>,
    /// What the line measures, e.g. "Copy rate". The start of the accessible text.
    pub aria_label: AttrValue,
    /// Shown before the line, e.g. "Copy rate". With it, the latest value and the range are shown
    /// after the line, so the line says what it draws without a hover. Absent: the line alone.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Shown after each figure, e.g. "rows/s".
    #[prop_or_default]
    pub unit: Option<AttrValue>,
    /// The unit as it is read aloud, e.g. "rows per second". Defaults to `unit`.
    #[prop_or_default]
    pub unit_words: Option<AttrValue>,
    /// How long the values span, shown after the range, e.g. "last 5 min".
    #[prop_or_default]
    pub span: Option<AttrValue>,
    /// The span as it is read aloud, e.g. "last 5 minutes". Defaults to `span`.
    #[prop_or_default]
    pub span_words: Option<AttrValue>,
    /// When each value was taken, one per value in `values` (e.g. "3 min ago", "now"); shown
    /// with the value of the point a hover, a tap or the keyboard picks. Absent: the value alone.
    #[prop_or_default]
    pub point_labels: Vec<AttrValue>,
    /// Width in CSS pixels.
    #[prop_or(96)]
    pub width: u32,
    /// Height in CSS pixels.
    #[prop_or(24)]
    pub height: u32,
    /// Any CSS colour of the line. The default, `currentColor`, follows the surrounding text, so
    /// it follows the theme and forced colours; a variable such as `var(--bulma-link-on-scheme)`
    /// (the text-safe shade in both schemes, above 3:1 against the page) works too.
    #[prop_or_else(|| AttrValue::from("currentColor"))]
    pub color: AttrValue,
    /// Formats each full figure: the latest value, a point's value and the accessible text.
    /// Defaults to thousands separators and at most one decimal ([`format_number`]).
    #[prop_or_default]
    pub format_value: Option<Callback<f64, String>>,
    /// Formats the two figures of the shown range. Defaults to a compact figure: "980", "1.4k", "48k", "2.5M".
    #[prop_or_default]
    pub format_short: Option<Callback<f64, String>>,
    /// Writes the whole accessible text from the figures, in the caller's words. Defaults to
    /// `"<aria_label>, <span_words>: from <min> to <max> <unit_words>, now <last>"`.
    #[prop_or_default]
    pub describe: Option<Callback<SparklineStats, String>>,
    /// Said after `aria_label` when there is nothing to draw.
    #[prop_or_else(|| AttrValue::from("No data"))]
    pub empty_label: AttrValue,
    /// Rendered as `data-testid` on the line; its parts carry `<testid>-label`, `-now`, `-range`
    /// and `-tip`. Absent: nothing is rendered.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
}

/// A small line of how a value moved, read by a screen reader as one image with a text summary.
///
/// No axes, no gridlines: it sits beside the number it explains. The vertical range is the series'
/// own minimum to maximum, its first lowest and highest points marked by a hollow ring; equal
/// values draw a flat line through the middle, a single value a dot, and an empty series a dashed line.
#[component(Sparkline)]
pub fn sparkline(props: &SparklineProps) -> Html {
    let active = use_state_eq(|| None::<usize>);
    let line_ref = use_node_ref();
    use_escape_closes(active.clone());
    let points = drawn_points(&props.values, &props.point_labels);
    let values: Vec<f64> = points.iter().map(|point| point.value).collect();
    let frame = Frame {
        width: f64::from(props.width),
        height: f64::from(props.height),
    };
    let series_stats = stats(&values);
    let positions = series_stats.map(|stats| frame.points(&values, &stats)).unwrap_or_default();
    let shown = (*active).filter(|index| *index < points.len());
    let plot = Plot {
        points: &points,
        positions: &positions,
        frame: &frame,
        shown,
    };
    html! {
        <span class={classes!("ybc-sparkline", props.classes.clone())}>
            <style>{SPARKLINE_STYLE}</style>
            { render_label(props) }
            <span class="ybc-sparkline-plot" onmouseleave={close(&active)}>
                { render_line(props, &plot, series_stats.as_ref(), &active, &line_ref) }
                { render_tip(props, &plot) }
            </span>
            <span class="is-sr-only" aria-live="polite">{ shown.map(|index| spoken_point(props, &points[index])).unwrap_or_default() }</span>
            { series_stats.map(|stats| render_caption(props, &stats)).unwrap_or_default() }
        </span>
    }
}

/// What one render knows about the drawn points, and which one is shown.
struct Plot<'plot> {
    points: &'plot [Point],
    positions: &'plot [(f64, f64)],
    frame: &'plot Frame,
    shown: Option<usize>,
}

fn render_line(
    props: &SparklineProps, plot: &Plot, series_stats: Option<&SparklineStats>, active: &UseStateHandle<Option<usize>>, line_ref: &NodeRef,
) -> Html {
    let summary = summarize(props, series_stats);
    let shape = match series_stats {
        Some(stats) => render_series(props, plot, stats),
        None => render_empty(plot.frame),
    };
    let count = plot.points.len();
    html! {
        <svg ref={line_ref.clone()} class="ybc-sparkline-line" role="img" aria-label={summary}
             tabindex={(count > 0).then_some("0")}
             width={props.width.to_string()} height={props.height.to_string()}
             viewBox={format!("0 0 {} {}", props.width, props.height)} data-testid={props.testid.clone()}
             onfocus={show_latest(active, count)} onblur={close(active)} onkeydown={move_by_key(active, count)}
             onmousemove={show_nearest(active, line_ref, plot)} onclick={show_nearest(active, line_ref, plot)}>
            { shape }
        </svg>
    }
}

fn close<Event: 'static>(active: &UseStateHandle<Option<usize>>) -> Callback<Event> {
    let active = active.clone();
    Callback::from(move |_: Event| active.set(None))
}

fn show_latest(active: &UseStateHandle<Option<usize>>, count: usize) -> Callback<FocusEvent> {
    let active = active.clone();
    Callback::from(move |_: FocusEvent| active.set(count.checked_sub(1)))
}

/// Only the key is read, never the event's target: Yew delegates listeners to the app root.
fn move_by_key(active: &UseStateHandle<Option<usize>>, count: usize) -> Callback<KeyboardEvent> {
    let active = active.clone();
    Callback::from(move |event: KeyboardEvent| {
        let Some(next) = moved_by_key(&event.key(), *active, count) else {
            return;
        };
        event.prevent_default();
        active.set(Some(next));
    })
}

/// The pointer's place is measured against the line's own box (its `NodeRef`), never against
/// `current_target()`, which is the app root under Yew's delegated listeners.
fn show_nearest(active: &UseStateHandle<Option<usize>>, line_ref: &NodeRef, plot: &Plot) -> Callback<MouseEvent> {
    let active = active.clone();
    let line_ref = line_ref.clone();
    let positions = plot.positions.to_vec();
    let drawn_width = plot.frame.width;
    Callback::from(move |event: MouseEvent| {
        let Some(line) = line_ref.cast::<web_sys::Element>() else {
            return;
        };
        let bounds = line.get_bounding_client_rect();
        if bounds.width() <= 0.0 {
            return;
        }
        let x = (f64::from(event.client_x()) - bounds.left()) * drawn_width / bounds.width();
        active.set(nearest_index(x, &positions));
    })
}

/// While a point is shown, Escape anywhere closes it: a hover tooltip is dismissible without
/// moving the pointer (WCAG 1.4.13). The listener lives only while something is shown.
#[hook]
fn use_escape_closes(active: UseStateHandle<Option<usize>>) {
    let open = active.is_some();
    use_effect_with(open, move |open| {
        let listener = open.then(|| EscapeListener::attach(active));
        move || drop(listener)
    });
}

struct EscapeListener {
    document: web_sys::Document,
    on_key: Closure<dyn Fn(web_sys::KeyboardEvent)>,
}

impl EscapeListener {
    fn attach(active: UseStateHandle<Option<usize>>) -> Option<Self> {
        let document = web_sys::window()?.document()?;
        let on_key = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                active.set(None);
            }
        });
        document
            .add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref())
            .ok()?;
        Some(Self { document, on_key })
    }
}

impl Drop for EscapeListener {
    fn drop(&mut self) {
        let _ = self
            .document
            .remove_event_listener_with_callback("keydown", self.on_key.as_ref().unchecked_ref());
    }
}

fn full_format(props: &SparklineProps) -> Callback<f64, String> {
    props.format_value.clone().unwrap_or_else(|| Callback::from(format_number))
}

fn spoken_unit(props: &SparklineProps) -> Option<&AttrValue> {
    props.unit_words.as_ref().or(props.unit.as_ref())
}

fn summarize(props: &SparklineProps, series_stats: Option<&SparklineStats>) -> String {
    let Some(stats) = series_stats else {
        return format!("{}: {}", props.aria_label, props.empty_label);
    };
    if let Some(describe) = &props.describe {
        return describe.emit(*stats);
    }
    let words = SummaryWords {
        name: &props.aria_label,
        span: props.span_words.as_ref().or(props.span.as_ref()),
        unit: spoken_unit(props),
    };
    let format = full_format(props);
    summary_text(stats, &words, &|value| format.emit(value))
}

fn spoken_point(props: &SparklineProps, point: &Point) -> String {
    point_text(point, SPOKEN_SEPARATOR, full_format(props).emit(point.value), spoken_unit(props))
}

/// A part's test id: `<testid>-<part>`, or nothing without a `testid`.
fn part_testid(props: &SparklineProps, part: &str) -> Option<AttrValue> {
    props.testid.as_ref().map(|testid| AttrValue::from(format!("{testid}-{part}")))
}

/// The label, latest value and range repeat the image's summary, so a screen reader skips them.
fn render_label(props: &SparklineProps) -> Html {
    let Some(label) = &props.label else {
        return Html::default();
    };
    html! { <span class="ybc-sparkline-label" aria-hidden="true" data-testid={part_testid(props, "label")}>{ label.clone() }</span> }
}

fn render_caption(props: &SparklineProps, stats: &SparklineStats) -> Html {
    if props.label.is_none() {
        return Html::default();
    }
    let latest = with_unit(full_format(props).emit(stats.last), props.unit.as_ref());
    let short = props.format_short.clone().unwrap_or_else(|| Callback::from(compact_number));
    let range = range_text(stats, &|value| short.emit(value), props.unit.as_ref(), props.span.as_ref());
    html! {
        <>
            <span class="ybc-sparkline-now" aria-hidden="true" data-testid={part_testid(props, "now")}>{ latest }</span>
            if let Some(range) = range {
                <span class="ybc-sparkline-range" aria-hidden="true" data-testid={part_testid(props, "range")}>{ range }</span>
            }
        </>
    }
}

/// The shown point's time and value above it, kept inside the plot so the pointer may move onto it.
fn render_tip(props: &SparklineProps, plot: &Plot) -> Html {
    let Some(index) = plot.shown else {
        return Html::default();
    };
    let (x, _) = plot.positions[index];
    let text = point_text(
        &plot.points[index],
        SHOWN_SEPARATOR,
        full_format(props).emit(plot.points[index].value),
        props.unit.as_ref(),
    );
    html! {
        <span class="ybc-sparkline-tip" aria-hidden="true" style={tip_anchor(x, plot.frame.width)} data-testid={part_testid(props, "tip")}>
            <span>{ text }</span>
        </span>
    }
}

fn tip_anchor(x: f64, width: f64) -> String {
    if x < width / ANCHOR_THIRDS {
        "left:0".to_string()
    } else if x > width - width / ANCHOR_THIRDS {
        "right:0".to_string()
    } else {
        format!("left:{x:.2}px;transform:translateX(-50%)")
    }
}

fn polyline_points(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{x:.2},{y:.2}")).collect::<Vec<_>>().join(" ")
}

fn render_series(props: &SparklineProps, plot: &Plot, stats: &SparklineStats) -> Html {
    let values: Vec<f64> = plot.points.iter().map(|point| point.value).collect();
    let extreme_marks = extremes(&values, stats)
        .map(|(lowest, highest)| html! { <>{ ring(plot.positions[lowest], &props.color) }{ ring(plot.positions[highest], &props.color) }</> })
        .unwrap_or_default();
    let last_point = plot.positions.last().copied();
    html! {
        <>
            if plot.positions.len() > 1 {
                <polyline points={polyline_points(plot.positions)} fill="none" stroke={props.color.clone()} stroke-width={STROKE_WIDTH}
                          stroke-linejoin="round" stroke-linecap="round" />
            }
            { extreme_marks }
            if let Some((x, y)) = last_point {
                <circle class="is-filled" cx={format!("{x:.2}")} cy={format!("{y:.2}")} r={LAST_POINT_RADIUS} fill={props.color.clone()} />
            }
            if let Some(index) = plot.shown {
                <circle class="ybc-sparkline-active" cx={format!("{:.2}", plot.positions[index].0)} cy={format!("{:.2}", plot.positions[index].1)}
                        r={ACTIVE_RADIUS} fill="none" stroke={props.color.clone()} stroke-width={STROKE_WIDTH} />
            }
        </>
    }
}

/// A hollow ring: the lowest or highest point.
fn ring((x, y): (f64, f64), color: &AttrValue) -> Html {
    html! {
        <circle class="ybc-sparkline-extreme" cx={format!("{x:.2}")} cy={format!("{y:.2}")} r={EXTREME_RADIUS}
                fill={SURFACE_COLOR} stroke={color.clone()} stroke-width={EXTREME_STROKE_WIDTH} />
    }
}

fn render_empty(frame: &Frame) -> Html {
    let middle = format!("{:.2}", frame.height / 2.0);
    html! {
        <line x1={format!("{INSET:.2}")} y1={middle.clone()} x2={format!("{:.2}", frame.width - INSET)} y2={middle}
              stroke="currentColor" stroke-width={STROKE_WIDTH} stroke-dasharray={EMPTY_DASHES} stroke-opacity={EMPTY_OPACITY} />
    }
}
