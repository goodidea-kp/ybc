//! The sparkline, rendered to HTML on the host (Bastion BASTION-697, BASTION-781):
//! inline SVG in the text colour, one focusable image whose name summarises the
//! series, a visible label, latest value and range when asked for, and no failure
//! on an empty, single, flat or NaN-ridden series.

use ybc::{Sparkline, SparklineProps, SparklineStats};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render(props: SparklineProps) -> String {
    LocalServerRenderer::<Sparkline>::with_props(props).hydratable(false).render().await
}

fn copy_rate(values: Vec<f64>) -> SparklineProps {
    yew::props!(SparklineProps {
        values,
        aria_label: "Copy rate",
        testid: Some(AttrValue::from("copy-rate")),
    })
}

#[tokio::test]
async fn a_series_is_one_focusable_image_named_by_its_range_and_latest_value() {
    let html = render(copy_rate(vec![1200.0, 980.0, 1410.5, 1350.0])).await;

    assert!(html.contains(r#"class="ybc-sparkline-line""#), "{html}");
    assert!(html.contains(r#"role="img""#), "{html}");
    assert!(html.contains(r#"tabindex="0""#), "{html}");
    assert!(
        html.contains(r#"aria-label="Copy rate: between 980 and 1,410.5, latest 1,350""#),
        "{html}"
    );
    assert!(html.contains(r#"data-testid="copy-rate""#), "{html}");
    assert!(html.contains("<polyline"), "{html}");
    assert!(html.contains("<circle"), "the latest value is marked: {html}");
}

#[tokio::test]
async fn the_line_follows_the_text_colour_unless_told_otherwise() {
    let default = render(copy_rate(vec![1.0, 2.0])).await;
    let mut themed = copy_rate(vec![1.0, 2.0]);
    themed.color = "var(--bulma-link)".into();
    let themed = render(themed).await;

    assert!(default.contains(r#"stroke="currentColor""#), "{default}");
    assert!(themed.contains(r#"stroke="var(--bulma-link)""#), "{themed}");
}

#[tokio::test]
async fn width_and_height_size_the_svg_and_its_view_box() {
    let mut props = copy_rate(vec![1.0, 2.0, 3.0]);
    props.width = 120;
    props.height = 30;
    let html = render(props).await;

    assert!(html.contains(r#"width="120""#), "{html}");
    assert!(html.contains(r#"height="30""#), "{html}");
    assert!(html.contains(r#"viewBox="0 0 120 30""#), "{html}");
}

#[tokio::test]
async fn the_callers_formatter_writes_each_figure() {
    let mut props = copy_rate(vec![10.0, 30.0, 20.0]);
    props.format_value = Some(Callback::from(|value: f64| format!("{value:.0} rows/min")));
    let html = render(props).await;

    assert!(
        html.contains(r#"aria-label="Copy rate: between 10 rows/min and 30 rows/min, latest 20 rows/min""#),
        "{html}"
    );
}

#[tokio::test]
async fn the_caller_may_write_the_whole_summary_in_its_own_words() {
    let mut props = copy_rate(vec![10.0, 30.0, 20.0]);
    props.describe = Some(Callback::from(|stats: SparklineStats| {
        format!("{} samples, now {}", stats.count, stats.last)
    }));
    let html = render(props).await;

    assert!(html.contains(r#"aria-label="3 samples, now 20""#), "{html}");
}

#[tokio::test]
async fn an_empty_series_says_so_and_draws_a_dashed_line() {
    let html = render(copy_rate(Vec::new())).await;

    assert!(html.contains(r#"aria-label="Copy rate: No data""#), "{html}");
    assert!(!html.contains("tabindex"), "nothing to move between, so no tab stop: {html}");
    assert!(html.contains("stroke-dasharray"), "{html}");
    assert!(!html.contains("<polyline"), "{html}");
}

#[tokio::test]
async fn nan_and_infinite_values_are_left_out() {
    let only_nan = render(copy_rate(vec![f64::NAN, f64::INFINITY])).await;
    let mixed = render(copy_rate(vec![f64::NAN, 5.0, f64::NAN, 7.0, f64::NEG_INFINITY])).await;

    assert!(only_nan.contains(r#"aria-label="Copy rate: No data""#), "{only_nan}");
    assert!(mixed.contains(r#"aria-label="Copy rate: between 5 and 7, latest 7""#), "{mixed}");
    assert!(!mixed.contains("NaN"), "{mixed}");
}

#[tokio::test]
async fn a_single_value_is_a_dot_without_a_line() {
    let html = render(copy_rate(vec![42.0])).await;

    assert!(html.contains(r#"aria-label="Copy rate: latest 42""#), "{html}");
    assert!(html.contains("<circle"), "{html}");
    assert!(!html.contains("<polyline"), "{html}");
}

#[tokio::test]
async fn equal_values_draw_a_flat_line_through_the_middle() {
    let html = render(copy_rate(vec![3.0, 3.0, 3.0])).await;

    assert!(html.contains(r#"points="2.50,12.00 48.00,12.00 93.50,12.00""#), "{html}");
}

fn copy_rate_said_in_words(values: Vec<f64>) -> SparklineProps {
    let mut props = copy_rate(values);
    props.label = Some("Copy rate".into());
    props.unit = Some("rows/s".into());
    props.unit_words = Some("rows per second".into());
    props.span = Some("last 3 min".into());
    props.span_words = Some("last 3 minutes".into());
    props.point_labels = vec!["2 min ago".into(), "1 min ago".into(), "latest".into()];
    props.format_value = Some(Callback::from(|value: f64| ybc::format_number(value.round())));
    props
}

#[tokio::test]
async fn a_label_shows_what_the_line_draws_without_a_hover() {
    let html = render(copy_rate_said_in_words(vec![48_000.0, 71_000.0, 56_590.4])).await;

    let label = html.find(r#"data-testid="copy-rate-label""#).expect("a visible label");
    let line = html.find("<svg").expect("the line");
    let latest = html.find(r#"data-testid="copy-rate-latest""#).expect("the latest value");
    assert!(label < line && line < latest, "label, line, latest value in reading order: {html}");
    assert!(html.contains(">Copy rate</span>"), "{html}");
    assert!(html.contains(">56,590 rows/s</span>"), "{html}");
    assert!(
        html.contains(">(48k\u{2013}71k rows/s, last 3 min)</span>"),
        "the range apart from the latest value: {html}"
    );
}

#[tokio::test]
async fn the_summary_says_the_window_in_words_a_screen_reader_reads() {
    let html = render(copy_rate_said_in_words(vec![48_000.0, 71_000.0, 56_590.4])).await;

    assert!(
        html.contains(r#"aria-label="Copy rate, last 3 minutes: between 48,000 and 71,000, latest 56,590 rows per second""#),
        "{html}"
    );
    assert!(
        html.contains(r#"<span aria-hidden="true" data-testid="copy-rate-label""#),
        "the image says it once: {html}"
    );
    assert!(
        html.contains(r#"<span aria-live="polite" class="is-sr-only"></span>"#),
        "the live region is silent until the keyboard moves to a point: {html}"
    );
}

#[tokio::test]
async fn without_a_label_only_the_line_is_shown() {
    let html = render(copy_rate(vec![1.0, 2.0, 3.0])).await;

    assert!(!html.contains("copy-rate-label"), "{html}");
    assert!(!html.contains("copy-rate-latest"), "{html}");
    assert!(!html.contains("copy-rate-range"), "{html}");
    assert!(!html.contains("copy-rate-tip"), "no tooltip before a hover or focus: {html}");
}

#[tokio::test]
async fn the_lowest_and_highest_points_are_marked() {
    let html = render(copy_rate(vec![3.0, 1.0, 9.0, 4.0])).await;

    assert_eq!(html.matches("ybc-sparkline-extreme").count(), 2, "{html}");
    let flat = render(copy_rate(vec![3.0, 3.0, 3.0])).await;
    assert!(!flat.contains("ybc-sparkline-extreme"), "{flat}");
}

#[tokio::test]
async fn its_style_goes_to_the_head_once_not_into_each_sparkline() {
    let html = render(copy_rate(vec![1.0, 2.0])).await;

    assert!(!html.contains("<style"), "{html}");
}
