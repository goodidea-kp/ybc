//! The sparkline, rendered to HTML on the host (Bastion BASTION-697): inline
//! SVG in the text colour, one image whose name summarises the series, and no
//! failure on an empty, single, flat or NaN-ridden series.

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
async fn a_series_is_one_image_named_by_its_last_min_and_max() {
    let html = render(copy_rate(vec![1200.0, 980.0, 1410.5, 1350.0])).await;

    assert!(html.starts_with("<svg"), "{html}");
    assert!(html.contains(r#"role="img""#), "{html}");
    assert!(html.contains(r#"aria-label="Copy rate: last 1,350, min 980, max 1,410.5""#), "{html}");
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
        html.contains(r#"aria-label="Copy rate: last 20 rows/min, min 10 rows/min, max 30 rows/min""#),
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
    assert!(html.contains("stroke-dasharray"), "{html}");
    assert!(!html.contains("<polyline"), "{html}");
}

#[tokio::test]
async fn nan_and_infinite_values_are_left_out() {
    let only_nan = render(copy_rate(vec![f64::NAN, f64::INFINITY])).await;
    let mixed = render(copy_rate(vec![f64::NAN, 5.0, f64::NAN, 7.0, f64::NEG_INFINITY])).await;

    assert!(only_nan.contains(r#"aria-label="Copy rate: No data""#), "{only_nan}");
    assert!(mixed.contains(r#"aria-label="Copy rate: last 7, min 5, max 7""#), "{mixed}");
    assert!(!mixed.contains("NaN"), "{mixed}");
}

#[tokio::test]
async fn a_single_value_is_a_dot_without_a_line() {
    let html = render(copy_rate(vec![42.0])).await;

    assert!(html.contains(r#"aria-label="Copy rate: last 42, min 42, max 42""#), "{html}");
    assert!(html.contains("<circle"), "{html}");
    assert!(!html.contains("<polyline"), "{html}");
}

#[tokio::test]
async fn equal_values_draw_a_flat_line_through_the_middle() {
    let html = render(copy_rate(vec![3.0, 3.0, 3.0])).await;

    assert!(html.contains(r#"points="2.50,12.00 48.00,12.00 93.50,12.00""#), "{html}");
}
