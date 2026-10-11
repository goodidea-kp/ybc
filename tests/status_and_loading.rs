//! Status chips, skeletons, remote data and disclosures, rendered to HTML on
//! the host (Bastion BASTION-105): colour is never the only signal, a
//! placeholder is busy and still says something, and every state has a face.

use std::fmt;

use ybc::{
    Button, ButtonProps, Disclosure, DisclosureProps, Remote, RemoteView, RemoteViewProps, SkeletonBlock, SkeletonBlockProps, StatusTag,
    StatusTagProps, StatusTone, Tag, TagProps, Title, TitleProps,
};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render<C>(props: C::Properties) -> String
where
    C: BaseComponent,
{
    LocalServerRenderer::<C>::with_props(props).hydratable(false).render().await
}

fn status(tone: StatusTone, text: Option<&str>, icon: Option<&str>, title: Option<&str>) -> StatusTagProps {
    StatusTagProps {
        tone,
        icon: icon.map(AttrValue::from),
        title: title.map(AttrValue::from),
        light: false,
        size: None,
        id: None,
        testid: Some("state".into()),
        classes: Classes::new(),
        children: text.map_or_else(Children::default, |text| Children::new(vec![html! { { text.to_string() } }])),
    }
}

#[tokio::test]
async fn a_status_tag_with_text_is_read_as_its_text() {
    let html = render::<StatusTag>(status(StatusTone::Success, Some("ACTIVE"), None, None)).await;

    assert!(html.contains(r#"class="tag is-success""#), "{html}");
    assert!(html.contains("<span>ACTIVE</span>"), "{html}");
    assert!(!html.contains("role="), "{html}");
    assert!(html.contains(r#"data-testid="state""#), "{html}");
}

#[tokio::test]
async fn an_icon_beside_text_is_hidden_from_a_screen_reader() {
    let html = render::<StatusTag>(status(StatusTone::Warning, Some("EXPIRING"), Some("fas fa-clock"), Some("In 3 days"))).await;

    assert!(html.contains(r#"<i aria-hidden="true" class="fas fa-clock">"#), "{html}");
    assert!(html.contains(r#"title="In 3 days""#), "{html}");
    assert!(html.contains("<span>EXPIRING</span>"), "{html}");
}

#[tokio::test]
async fn an_icon_only_tag_is_an_image_named_by_its_title() {
    let html = render::<StatusTag>(status(StatusTone::Danger, None, Some("fas fa-ban"), Some("Revoked"))).await;

    assert!(html.contains(r#"role="img" aria-label="Revoked""#), "{html}");
    assert!(html.contains("fa-ban"), "{html}");
}

#[tokio::test]
async fn a_tag_with_neither_text_nor_icon_still_shows_the_tones_icon_and_name() {
    let html = render::<StatusTag>(status(StatusTone::Danger, None, None, None)).await;

    assert!(html.contains(StatusTone::Danger.icon()), "never colour alone: {html}");
    assert!(html.contains(r#"aria-label="Danger""#), "{html}");
}

#[tokio::test]
async fn light_and_neutral_tones_pick_their_classes() {
    let mut props = status(StatusTone::Info, Some("PENDING"), None, None);
    props.light = true;
    let light = render::<StatusTag>(props).await;
    let neutral = render::<StatusTag>(status(StatusTone::Neutral, Some("n/a"), None, None)).await;

    assert!(light.contains(r#"class="tag is-info is-light""#), "{light}");
    assert!(neutral.contains(r#"class="tag""#), "{neutral}");
}

fn tag(icon: Option<&str>, skeleton: bool) -> TagProps {
    TagProps {
        children: Children::new(vec![html! { "v1.2" }]),
        classes: Classes::new(),
        tag: "span".into(),
        onclick: Callback::noop(),
        rounded: false,
        delete: false,
        size: None,
        icon: icon.map(AttrValue::from),
        skeleton,
    }
}

#[tokio::test]
async fn a_plain_tag_renders_exactly_as_before() {
    let html = render::<Tag>(tag(None, false)).await;

    assert_eq!(html, r#"<span class="tag">v1.2</span>"#);
}

#[tokio::test]
async fn a_tag_icon_comes_first_and_is_hidden() {
    let html = render::<Tag>(tag(Some("fas fa-tag"), false)).await;

    assert!(
        html.contains(r#"<span class="icon"><i aria-hidden="true" class="fas fa-tag"></i></span><span>v1.2</span>"#),
        "{html}"
    );
}

#[tokio::test]
async fn a_skeleton_tag_is_busy() {
    let html = render::<Tag>(tag(None, true)).await;

    assert!(
        html.contains(r#"class="tag is-skeleton""#) && html.contains(r#"aria-busy="true""#),
        "{html}"
    );
}

fn title(skeleton: bool) -> TitleProps {
    TitleProps {
        children: Children::new(vec![html! { "Domains" }]),
        classes: Classes::new(),
        tag: "h1".into(),
        is_spaced: false,
        size: None,
        skeleton,
    }
}

#[tokio::test]
async fn a_skeleton_title_is_busy_and_keeps_its_text() {
    let skeleton = render::<Title>(title(true)).await;
    let plain = render::<Title>(title(false)).await;

    assert!(skeleton.contains("is-skeleton") && skeleton.contains(r#"aria-busy="true""#), "{skeleton}");
    assert!(skeleton.contains("Domains"), "{skeleton}");
    assert_eq!(plain, r#"<h1 class="title">Domains</h1>"#);
}

#[tokio::test]
async fn a_skeleton_button_is_busy_and_cannot_be_pressed() {
    let html = render::<Button>(ButtonProps {
        skeleton: true,
        children: Children::new(vec![html! { "Rotate" }]),
        ..button_defaults()
    })
    .await;
    let plain = render::<Button>(ButtonProps {
        children: Children::new(vec![html! { "Rotate" }]),
        ..button_defaults()
    })
    .await;

    assert!(html.contains("is-skeleton") && html.contains(r#"aria-busy="true""#), "{html}");
    assert!(html.contains("disabled"), "{html}");
    assert!(!plain.contains("aria-busy") && !plain.contains("disabled"), "{plain}");
}

fn button_defaults() -> ButtonProps {
    yew::props!(ButtonProps {})
}

#[tokio::test]
async fn skeleton_lines_are_busy_and_say_loading_to_a_screen_reader() {
    let html = render::<SkeletonBlock>(yew::props!(SkeletonBlockProps { lines: 4u32 })).await;

    assert!(
        html.contains(r#"class="skeleton-lines""#) && html.contains(r#"aria-busy="true""#),
        "{html}"
    );
    assert!(html.contains(r#"<span class="is-sr-only">Loading…</span>"#), "{html}");
    assert_eq!(html.matches("<div></div>").count(), 4, "{html}");
}

#[tokio::test]
async fn zero_lines_is_one_skeleton_block() {
    let html = render::<SkeletonBlock>(yew::props!(SkeletonBlockProps {
        lines: 0u32,
        label: "Loading the audit trail…"
    }))
    .await;

    assert!(html.contains(r#"class="skeleton-block""#), "{html}");
    assert!(html.contains("Loading the audit trail…"), "{html}");
    assert!(!html.contains("<div></div>"), "{html}");
}

#[derive(Clone, Debug, PartialEq)]
struct ApiError(&'static str);

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

type Rows = Vec<&'static str>;

fn remote_view(remote: Remote<Rows, ApiError>, on_retry: Option<Callback<()>>) -> RemoteViewProps<Rows, ApiError> {
    RemoteViewProps {
        remote,
        render: Callback::from(|rows: Rows| html! { <ul>{ for rows.iter().map(|row| html! { <li>{ *row }</li> }) }</ul> }),
        skeleton: None,
        idle: Html::default(),
        render_failure: None,
        on_retry,
        testid: Some("domains".into()),
        classes: Classes::new(),
    }
}

#[tokio::test]
async fn loading_shows_a_busy_skeleton() {
    let html = render::<RemoteView<Rows, ApiError>>(remote_view(Remote::Loading, None)).await;

    assert!(html.starts_with(r#"<div aria-busy="true""#), "{html}");
    assert!(html.contains("skeleton-lines"), "{html}");
}

#[tokio::test]
async fn loaded_shows_the_data_and_is_no_longer_busy() {
    let html = render::<RemoteView<Rows, ApiError>>(remote_view(Remote::Loaded(vec!["alpha", "beta"]), None)).await;

    assert!(html.contains(r#"aria-busy="false""#), "{html}");
    assert!(html.contains("<li>alpha</li><li>beta</li>"), "{html}");
    assert!(!html.contains("skeleton"), "{html}");
}

#[tokio::test]
async fn failed_is_an_alert_with_the_error_and_a_retry_button() {
    let html = render::<RemoteView<Rows, ApiError>>(remote_view(
        Remote::Failed(ApiError("The server did not answer.")),
        Some(Callback::noop()),
    ))
    .await;

    assert!(html.contains(r#"role="alert""#), "{html}");
    assert!(html.contains("The server did not answer."), "{html}");
    assert!(html.contains(r#"data-testid="domains-failed""#), "{html}");
    assert!(html.contains(r#"data-testid="domains-retry""#) && html.contains("Try again"), "{html}");
}

#[tokio::test]
async fn failed_without_retry_has_no_button_and_idle_shows_nothing() {
    let failed = render::<RemoteView<Rows, ApiError>>(remote_view(Remote::Failed(ApiError("No.")), None)).await;
    let idle = render::<RemoteView<Rows, ApiError>>(remote_view(Remote::Idle, None)).await;

    assert!(!failed.contains("<button"), "{failed}");
    assert_eq!(idle, r#"<div aria-busy="false" data-testid="domains"></div>"#);
}

#[tokio::test]
async fn a_custom_skeleton_replaces_the_default_one() {
    let mut props = remote_view(Remote::Loading, None);
    props.skeleton = Some(html! { <p class="custom-skeleton">{ "…" }</p> });
    let html = render::<RemoteView<Rows, ApiError>>(props).await;

    assert!(html.contains("custom-skeleton") && !html.contains("skeleton-lines"), "{html}");
}

#[test]
fn remote_has_exactly_four_states_and_reads_like_one_value() {
    let loaded: Remote<u32, ApiError> = Ok(7).into();
    let failed: Remote<u32, ApiError> = Err(ApiError("down")).into();

    assert_eq!(loaded.loaded(), Some(&7));
    assert_eq!(loaded.clone().map(|count| count * 2), Remote::Loaded(14));
    assert_eq!(failed.failure(), Some(&ApiError("down")));
    assert!(Remote::<u32, ApiError>::Loading.is_loading());
    assert_eq!(Remote::<u32>::default(), Remote::Idle);
    assert_eq!(loaded.as_ref().loaded(), Some(&&7));
}

fn disclosure(open: bool) -> DisclosureProps {
    DisclosureProps {
        summary: html! { "Advanced" },
        open,
        on_toggle: None,
        id: None,
        testid: Some("advanced".into()),
        classes: Classes::new(),
        children: Children::new(vec![html! { <p>{ "Raw config" }</p> }]),
    }
}

#[tokio::test]
async fn a_disclosure_is_a_native_details_with_a_summary() {
    let html = render::<Disclosure>(disclosure(false)).await;

    assert!(html.starts_with("<details"), "{html}");
    assert!(
        html.contains(r#"data-testid="advanced-summary""#) && html.contains(">Advanced</summary>"),
        "{html}"
    );
    assert!(html.contains("Raw config"), "{html}");
    assert!(!html.contains("open"), "closed by default: {html}");
    assert!(html.contains("min-height: 24px"), "{html}");
}

#[tokio::test]
async fn an_open_disclosure_renders_open() {
    let html = render::<Disclosure>(disclosure(true)).await;

    assert!(html.contains("open"), "{html}");
}
