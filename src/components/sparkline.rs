//! A word-sized trend line drawn as inline SVG: no chart library, no script.
//!
//! ```rust,ignore
//! let rows_per_minute = vec![1200.0, 1350.0, 980.0, 1410.0];
//! let per_minute = Callback::from(|value: f64| format!("{value:.0} rows/min"));
//! html! {
//!     <ybc::Sparkline values={rows_per_minute} aria_label="Copy rate" format_value={per_minute} />
//! }
//! ```
use yew::prelude::*;

use crate::components::chart::format_number;

/// Room left around the line so the stroke and the last-point dot are never clipped.
const INSET: f64 = 2.5;
const STROKE_WIDTH: &str = "1.5";
const LAST_POINT_RADIUS: &str = "2";
const EMPTY_DASHES: &str = "2 3";
const EMPTY_OPACITY: &str = "0.4";
const SVG_STYLE: &str = "display:inline-block;vertical-align:middle;overflow:visible";

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
    /// Width in CSS pixels.
    #[prop_or(96)]
    pub width: u32,
    /// Height in CSS pixels.
    #[prop_or(24)]
    pub height: u32,
    /// Any CSS colour of the line. The default, `currentColor`, follows the surrounding text, so
    /// it follows the theme and forced colours; a variable such as `var(--bulma-link-on-scheme)` (the text-safe shade in both schemes) works too.
    #[prop_or_else(|| AttrValue::from("currentColor"))]
    pub color: AttrValue,
    /// Formats each figure of the accessible text, e.g. with a unit. Defaults to thousands
    /// separators and at most one decimal.
    #[prop_or_default]
    pub format_value: Option<Callback<f64, String>>,
    /// Writes the whole accessible text from the figures, in the caller's words. Defaults to
    /// `"<aria_label>: last <last>, min <min>, max <max>"`.
    #[prop_or_default]
    pub describe: Option<Callback<SparklineStats, String>>,
    /// Said after `aria_label` when there is nothing to draw.
    #[prop_or_else(|| AttrValue::from("No data"))]
    pub empty_label: AttrValue,
    /// Rendered as `data-testid`. Absent: nothing is rendered.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
}

/// A small line of how a value moved, read by a screen reader as one image with a text summary.
///
/// No axes, no hover: it sits beside the number it explains. The vertical range is the series'
/// own minimum to maximum; equal values draw a flat line through the middle, a single value a
/// dot, and an empty series a dashed line.
#[component(Sparkline)]
pub fn sparkline(props: &SparklineProps) -> Html {
    let values: Vec<f64> = props.values.iter().copied().filter(|value| value.is_finite()).collect();
    let frame = Frame {
        width: f64::from(props.width),
        height: f64::from(props.height),
    };
    let summary = summarize(&values, props);
    let shape = match stats(&values) {
        Some(stats) => render_series(&frame.points(&values, &stats), &props.color),
        None => render_empty(&frame),
    };
    html! {
        <svg class={classes!("ybc-sparkline", props.classes.clone())} role="img" aria-label={summary}
             width={props.width.to_string()} height={props.height.to_string()}
             viewBox={format!("0 0 {} {}", props.width, props.height)} style={SVG_STYLE} data-testid={props.testid.clone()}>
            { shape }
        </svg>
    }
}

/// The drawing area, in CSS pixels.
struct Frame {
    width: f64,
    height: f64,
}

impl Frame {
    /// Each value's position: spread evenly left to right, the last one at the right edge;
    /// the maximum at the top and the minimum at the bottom.
    fn points(&self, values: &[f64], stats: &SparklineStats) -> Vec<(f64, f64)> {
        let usable_width = (self.width - 2.0 * INSET).max(0.0);
        let step = if values.len() > 1 {
            usable_width / (values.len() - 1) as f64
        } else {
            0.0
        };
        let first_x = if values.len() > 1 { INSET } else { self.width - INSET };
        values
            .iter()
            .enumerate()
            .map(|(index, value)| (first_x + step * index as f64, self.y_of(*value, stats)))
            .collect()
    }

    fn y_of(&self, value: f64, stats: &SparklineStats) -> f64 {
        let range = stats.max - stats.min;
        if range <= 0.0 {
            return self.height / 2.0;
        }
        let usable_height = (self.height - 2.0 * INSET).max(0.0);
        INSET + (stats.max - value) / range * usable_height
    }
}

fn stats(values: &[f64]) -> Option<SparklineStats> {
    let last = *values.last()?;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Some(SparklineStats {
        min,
        max,
        last,
        count: values.len(),
    })
}

fn summarize(values: &[f64], props: &SparklineProps) -> String {
    let Some(stats) = stats(values) else {
        return format!("{}: {}", props.aria_label, props.empty_label);
    };
    if let Some(describe) = &props.describe {
        return describe.emit(stats);
    }
    let format = props.format_value.clone().unwrap_or_else(|| Callback::from(format_number));
    format!(
        "{}: last {}, min {}, max {}",
        props.aria_label,
        format.emit(stats.last),
        format.emit(stats.min),
        format.emit(stats.max)
    )
}

fn polyline_points(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{x:.2},{y:.2}")).collect::<Vec<_>>().join(" ")
}

fn render_series(points: &[(f64, f64)], color: &AttrValue) -> Html {
    let last_point = points.last().copied();
    html! {
        <>
            if points.len() > 1 {
                <polyline points={polyline_points(points)} fill="none" stroke={color.clone()} stroke-width={STROKE_WIDTH}
                          stroke-linejoin="round" stroke-linecap="round" />
            }
            if let Some((x, y)) = last_point {
                <circle cx={format!("{x:.2}")} cy={format!("{y:.2}")} r={LAST_POINT_RADIUS} fill={color.clone()} />
            }
        </>
    }
}

fn render_empty(frame: &Frame) -> Html {
    let middle = format!("{:.2}", frame.height / 2.0);
    html! {
        <line x1={format!("{INSET:.2}")} y1={middle.clone()} x2={format!("{:.2}", frame.width - INSET)} y2={middle}
              stroke="currentColor" stroke-width={STROKE_WIDTH} stroke-dasharray={EMPTY_DASHES} stroke-opacity={EMPTY_OPACITY} />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME: Frame = Frame { width: 100.0, height: 20.0 };

    fn points_of(values: &[f64]) -> Vec<(f64, f64)> {
        let stats = stats(values).unwrap_or(SparklineStats {
            min: 0.0,
            max: 0.0,
            last: 0.0,
            count: 0,
        });
        FRAME.points(values, &stats)
    }

    #[test]
    fn the_maximum_is_at_the_top_and_the_minimum_at_the_bottom() {
        let points = points_of(&[1.0, 3.0, 2.0]);
        assert_eq!(points, vec![(INSET, 20.0 - INSET), (50.0, INSET), (100.0 - INSET, 10.0)]);
    }

    #[test]
    fn equal_values_draw_a_flat_line_through_the_middle() {
        let points = points_of(&[7.0, 7.0, 7.0]);
        assert!(points.iter().all(|(_, y)| (*y - 10.0).abs() < 1e-9), "{points:?}");
    }

    #[test]
    fn a_single_value_sits_at_the_right_edge_in_the_middle() {
        assert_eq!(points_of(&[42.0]), vec![(100.0 - INSET, 10.0)]);
    }

    #[test]
    fn no_values_have_no_stats() {
        assert_eq!(stats(&[]), None);
    }

    #[test]
    fn stats_cover_min_max_last_and_count() {
        let stats = stats(&[4.0, -1.0, 9.0, 2.0]);
        assert_eq!(
            stats,
            Some(SparklineStats {
                min: -1.0,
                max: 9.0,
                last: 2.0,
                count: 4
            })
        );
    }
}
