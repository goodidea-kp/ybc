//! The arithmetic and the words of a [`Sparkline`](super::sparkline::Sparkline): where each point
//! sits, which keys move between points, and the texts it shows and says. No Yew in here, so all of
//! it is tested on the host.
use yew::AttrValue;

use super::chart::format_number;
use super::sparkline::SparklineStats;

/// Room left around the line so the stroke and the point marks are never clipped.
pub(super) const INSET: f64 = 2.5;
const THOUSAND: f64 = 1_000.0;
const MILLION: f64 = 1_000_000.0;
/// Below ten thousand (or ten million) a compact figure keeps one decimal: "1.4k", "48k".
const ONE_DECIMAL_BELOW: f64 = 10.0;
/// The fewest points for which marking the lowest and highest one says something the line does not.
const FEWEST_POINTS_FOR_EXTREMES: usize = 3;

/// One drawn value and, when the caller gave one, the words for when it was taken.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Point {
    pub value: f64,
    pub label: Option<AttrValue>,
}

/// The values paired with their labels, NaN and infinite values (and their labels) left out.
pub(super) fn drawn_points(values: &[f64], labels: &[AttrValue]) -> Vec<Point> {
    values
        .iter()
        .enumerate()
        .filter(|(_, value)| value.is_finite())
        .map(|(index, value)| Point {
            value: *value,
            label: labels.get(index).cloned(),
        })
        .collect()
}

pub(super) fn stats(values: &[f64]) -> Option<SparklineStats> {
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

/// The first lowest and the first highest point, worth a mark only when the line has a shape.
pub(super) fn extremes(values: &[f64], stats: &SparklineStats) -> Option<(usize, usize)> {
    if values.len() < FEWEST_POINTS_FOR_EXTREMES || stats.max <= stats.min {
        return None;
    }
    let lowest = values.iter().position(|value| *value == stats.min)?;
    let highest = values.iter().position(|value| *value == stats.max)?;
    Some((lowest, highest))
}

/// The drawing area, in CSS pixels.
pub(super) struct Frame {
    pub width: f64,
    pub height: f64,
}

impl Frame {
    /// Each value's position: spread evenly left to right, the last one at the right edge;
    /// the maximum at the top and the minimum at the bottom.
    pub fn points(&self, values: &[f64], stats: &SparklineStats) -> Vec<(f64, f64)> {
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

/// The point nearest to `x` (in the frame's pixels): what a hover or a tap shows.
pub(super) fn nearest_index(x: f64, positions: &[(f64, f64)]) -> Option<usize> {
    positions
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| (left.0 - x).abs().total_cmp(&(right.0 - x).abs()))
        .map(|(index, _)| index)
}

/// The point a key moves to among `count` points: Left/Down back, Right/Up on, Home the first,
/// End the last. With no point shown yet, the move starts from the latest. Other keys: `None`.
pub(super) fn moved_by_key(key: &str, active: Option<usize>, count: usize) -> Option<usize> {
    let last = count.checked_sub(1)?;
    let current = active.unwrap_or(last).min(last);
    match key {
        "ArrowLeft" | "ArrowDown" => Some(current.saturating_sub(1)),
        "ArrowRight" | "ArrowUp" => Some((current + 1).min(last)),
        "Home" => Some(0),
        "End" => Some(last),
        _ => None,
    }
}

/// A figure in few characters, for the range: "980", "1.4k", "48k", "2.5M".
pub(super) fn compact_number(value: f64) -> String {
    let magnitude = value.abs();
    let (scaled, suffix) = if magnitude >= MILLION {
        (value / MILLION, "M")
    } else if magnitude >= THOUSAND {
        (value / THOUSAND, "k")
    } else {
        return format_number(value);
    };
    let text = if scaled.abs() < ONE_DECIMAL_BELOW {
        format!("{scaled:.1}")
    } else {
        format!("{scaled:.0}")
    };
    format!("{}{suffix}", text.trim_end_matches(".0"))
}

/// `" rows/s"` for a unit, nothing without one.
pub(super) fn with_unit(figure: String, unit: Option<&AttrValue>) -> String {
    match unit {
        Some(unit) => format!("{figure} {unit}"),
        None => figure,
    }
}

/// "48k–71k rows/s, last 5 min": the window's lowest to highest value and how long it spans.
/// One value has no range, so nothing.
pub(super) fn range_text(
    stats: &SparklineStats, short: &dyn Fn(f64) -> String, unit: Option<&AttrValue>, span: Option<&AttrValue>,
) -> Option<String> {
    if stats.count < 2 {
        return None;
    }
    let figures = if stats.max > stats.min {
        format!("{}\u{2013}{}", short(stats.min), short(stats.max))
    } else {
        short(stats.max)
    };
    let range = with_unit(figures, unit);
    Some(match span {
        Some(span) => format!("{range}, {span}"),
        None => range,
    })
}

/// The words of the accessible summary that the caller chose.
pub(super) struct SummaryWords<'words> {
    pub name: &'words str,
    pub span: Option<&'words AttrValue>,
    pub unit: Option<&'words AttrValue>,
}

/// "Copy rate, last 5 minutes: from 48,000 to 71,000 rows per second, now 56,590".
pub(super) fn summary_text(stats: &SparklineStats, words: &SummaryWords, format: &dyn Fn(f64) -> String) -> String {
    let head = match words.span {
        Some(span) => format!("{}, {span}", words.name),
        None => words.name.to_string(),
    };
    if stats.count < 2 {
        return format!("{head}: now {}", with_unit(format(stats.last), words.unit));
    }
    let range = with_unit(format!("from {} to {}", format(stats.min), format(stats.max)), words.unit);
    format!("{head}: {range}, now {}", format(stats.last))
}

/// "3 min ago · 56,590 rows/s" (or the value alone when the point has no label).
pub(super) fn point_text(point: &Point, separator: &str, figure: String, unit: Option<&AttrValue>) -> String {
    let value = with_unit(figure, unit);
    match &point.label {
        Some(label) => format!("{label}{separator}{value}"),
        None => value,
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

    fn stats_of(values: &[f64]) -> SparklineStats {
        stats(values).unwrap_or_else(|| panic!("no stats for {values:?}"))
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

    #[test]
    fn labels_follow_their_values_when_nan_is_left_out() {
        let labels = vec![AttrValue::from("2 min ago"), AttrValue::from("1 min ago"), AttrValue::from("now")];
        let points = drawn_points(&[5.0, f64::NAN, 7.0], &labels);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].label.as_deref(), Some("2 min ago"));
        assert_eq!(points[1].label.as_deref(), Some("now"));
        assert_eq!(drawn_points(&[1.0, 2.0], &[])[1].label, None);
    }

    #[test]
    fn extremes_are_marked_only_when_the_line_has_a_shape() {
        assert_eq!(extremes(&[3.0, 1.0, 9.0, 9.0], &stats_of(&[3.0, 1.0, 9.0, 9.0])), Some((1, 2)));
        assert_eq!(extremes(&[3.0, 3.0, 3.0], &stats_of(&[3.0, 3.0, 3.0])), None);
        assert_eq!(extremes(&[1.0, 2.0], &stats_of(&[1.0, 2.0])), None);
    }

    #[test]
    fn the_nearest_point_answers_a_hover_or_a_tap() {
        let positions = [(2.5, 0.0), (50.0, 0.0), (97.5, 0.0)];
        assert_eq!(nearest_index(0.0, &positions), Some(0));
        assert_eq!(nearest_index(30.0, &positions), Some(1));
        assert_eq!(nearest_index(200.0, &positions), Some(2));
        assert_eq!(nearest_index(10.0, &[]), None);
    }

    #[test]
    fn arrow_keys_move_between_points_and_stop_at_the_ends() {
        assert_eq!(moved_by_key("ArrowLeft", None, 5), Some(3), "starts from the latest");
        assert_eq!(moved_by_key("ArrowLeft", Some(0), 5), Some(0));
        assert_eq!(moved_by_key("ArrowRight", Some(4), 5), Some(4));
        assert_eq!(moved_by_key("ArrowUp", Some(1), 5), Some(2));
        assert_eq!(moved_by_key("ArrowDown", Some(1), 5), Some(0));
        assert_eq!(moved_by_key("Home", Some(3), 5), Some(0));
        assert_eq!(moved_by_key("End", Some(0), 5), Some(4));
        assert_eq!(moved_by_key("Enter", Some(0), 5), None);
        assert_eq!(moved_by_key("ArrowLeft", None, 0), None);
    }

    #[test]
    fn compact_numbers_keep_few_characters() {
        assert_eq!(compact_number(980.0), "980");
        assert_eq!(compact_number(1_400.0), "1.4k");
        assert_eq!(compact_number(2_000.0), "2k");
        assert_eq!(compact_number(48_120.0), "48k");
        assert_eq!(compact_number(71_000.0), "71k");
        assert_eq!(compact_number(2_500_000.0), "2.5M");
        assert_eq!(compact_number(-48_000.0), "-48k");
    }

    #[test]
    fn the_range_names_lowest_to_highest_with_unit_and_span() {
        let unit = AttrValue::from("rows/s");
        let span = AttrValue::from("last 5 min");
        let window = stats_of(&[48_000.0, 71_000.0, 56_590.0]);
        assert_eq!(
            range_text(&window, &compact_number, Some(&unit), Some(&span)).as_deref(),
            Some("48k\u{2013}71k rows/s, last 5 min")
        );
        assert_eq!(range_text(&window, &compact_number, None, None).as_deref(), Some("48k\u{2013}71k"));
        let flat = stats_of(&[3.0, 3.0]);
        assert_eq!(range_text(&flat, &compact_number, Some(&unit), None).as_deref(), Some("3 rows/s"));
        assert_eq!(range_text(&stats_of(&[3.0]), &compact_number, Some(&unit), None), None);
    }

    #[test]
    fn the_summary_says_the_window_and_the_latest_value() {
        let span = AttrValue::from("last 5 minutes");
        let unit = AttrValue::from("rows per second");
        let words = SummaryWords {
            name: "Copy rate",
            span: Some(&span),
            unit: Some(&unit),
        };
        let window = stats_of(&[48_000.0, 71_000.0, 56_590.0]);
        assert_eq!(
            summary_text(&window, &words, &format_number),
            "Copy rate, last 5 minutes: from 48,000 to 71,000 rows per second, now 56,590"
        );
        assert_eq!(
            summary_text(&stats_of(&[42.0]), &words, &format_number),
            "Copy rate, last 5 minutes: now 42 rows per second"
        );
    }

    #[test]
    fn a_point_reads_as_its_time_and_value() {
        let unit = AttrValue::from("rows/s");
        let labelled = Point {
            value: 1.0,
            label: Some(AttrValue::from("3 min ago")),
        };
        let bare = Point { value: 1.0, label: None };
        assert_eq!(
            point_text(&labelled, " \u{00b7} ", "56,590".into(), Some(&unit)),
            "3 min ago \u{00b7} 56,590 rows/s"
        );
        assert_eq!(point_text(&bare, ": ", "56,590".into(), None), "56,590");
    }
}
