//! The accessibility primitives, rendered to HTML on the host: what a screen
//! reader and a keyboard get from each one (Bastion BASTION-101).

use ybc::{
    Callout, CalloutProps, CalloutSeverity, CopyButton, CopyButtonProps, IconText, IconTextProps, Notification, NotificationProps, PaginationItem,
    PaginationItemProps, PaginationItemType, ReasonedButton, ReasonedButtonProps,
};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render<C>(props: C::Properties) -> String
where
    C: BaseComponent,
{
    LocalServerRenderer::<C>::with_props(props).hydratable(false).render().await
}

fn callout(severity: CalloutSeverity, title: Option<&str>) -> CalloutProps {
    CalloutProps {
        severity,
        title: title.map(AttrValue::from),
        compact: false,
        testid: Some("note".into()),
        id: None,
        quiet: false,
        classes: Classes::new(),
        children: Children::new(vec![html! { "Body text." }]),
    }
}

#[tokio::test]
async fn a_danger_callout_is_an_alert_and_the_rest_are_status() {
    let danger = render::<Callout>(callout(CalloutSeverity::Danger, None)).await;
    let warning = render::<Callout>(callout(CalloutSeverity::Warning, Some("Heads up"))).await;

    assert!(danger.contains(r#"role="alert""#), "{danger}");
    assert!(danger.contains("is-danger"), "{danger}");
    assert!(warning.contains(r#"role="status""#), "{warning}");
    assert!(warning.contains("Heads up") && warning.contains("Body text."), "{warning}");
    assert!(warning.contains(r#"data-testid="note""#), "{warning}");
}

#[tokio::test]
async fn every_callout_has_an_icon_a_screen_reader_skips() {
    for severity in [
        CalloutSeverity::Info,
        CalloutSeverity::Success,
        CalloutSeverity::Warning,
        CalloutSeverity::Danger,
    ] {
        let html = render::<Callout>(callout(severity, None)).await;
        assert!(html.contains(severity.icon()), "{html}");
        assert!(html.contains(r#"aria-hidden="true""#), "{html}");
    }
}

fn reasoned(reason: Option<&str>) -> ReasonedButtonProps {
    ReasonedButtonProps {
        disabled_reason: reason.map(AttrValue::from),
        id: "assign".into(),
        onclick: Callback::noop(),
        classes: Classes::new(),
        testid: Some("assign-btn".into()),
        reason_testid: Some("assign-why-not".into()),
        reason_elsewhere: None,
        children: Children::new(vec![html! { "Assign" }]),
    }
}

#[tokio::test]
async fn a_button_with_a_reason_stays_focusable_and_points_to_the_reason() {
    let html = render::<ReasonedButton>(reasoned(Some("Enable a policy first."))).await;

    assert!(html.contains("<button"), "{html}");
    assert!(html.contains(r#"aria-disabled="true""#), "{html}");
    assert!(html.contains(r#"aria-describedby="assign-why-not""#), "{html}");
    assert!(
        html.contains(r#"id="assign-why-not""#) && html.contains("Enable a policy first."),
        "{html}"
    );
    assert!(
        !html.contains(" disabled"),
        "never the disabled attribute, which drops it from the tab order: {html}"
    );
}

#[tokio::test]
async fn a_working_button_says_nothing_extra() {
    let html = render::<ReasonedButton>(reasoned(None)).await;

    assert!(!html.contains("aria-disabled") && !html.contains("aria-describedby"), "{html}");
    assert!(!html.contains("why-not"), "{html}");
}

#[tokio::test]
async fn icon_text_hides_its_icon_from_screen_readers() {
    let html = render::<IconText>(IconTextProps {
        icon: "fas fa-plus".into(),
        classes: Classes::new(),
        children: Children::new(vec![html! { "Add" }]),
    })
    .await;

    assert!(
        html.contains("icon-text") && html.contains(r#"aria-hidden="true""#) && html.contains("Add"),
        "{html}"
    );
}

#[tokio::test]
async fn a_copy_button_is_a_button_with_a_polite_live_region() {
    let html = render::<CopyButton>(CopyButtonProps {
        text: "secret".into(),
        label: "Copy SQL".into(),
        copied_label: "Copied".into(),
        aria_label: None,
        classes: Classes::new(),
        testid: Some("copy".into()),
    })
    .await;

    assert!(html.contains(r#"<button type="button""#) && html.contains("Copy SQL"), "{html}");
    assert!(html.contains(r#"aria-live="polite""#), "{html}");
    assert!(!html.contains("secret"), "the copied text is not rendered: {html}");
}

fn page(current: bool) -> PaginationItemProps {
    PaginationItemProps {
        children: Children::new(vec![html! { "3" }]),
        item_type: PaginationItemType::Link,
        label: "Page 3".into(),
        onclick: Callback::noop(),
        current,
    }
}

#[tokio::test]
async fn a_pagination_item_is_a_keyboard_reachable_button_and_marks_the_current_page() {
    let current = render::<PaginationItem>(page(true)).await;
    let other = render::<PaginationItem>(page(false)).await;

    assert!(current.contains("<button") && !current.contains("<a"), "{current}");
    assert!(current.contains(r#"aria-current="page""#), "{current}");
    assert!(!other.contains("aria-current"), "{other}");
}

#[tokio::test]
async fn a_notification_carries_the_role_it_is_given() {
    let html = render::<Notification>(NotificationProps {
        children: Children::new(vec![html! { "Saved." }]),
        classes: classes!("is-success"),
        role: Some("status".into()),
    })
    .await;

    assert!(html.contains(r#"role="status""#), "{html}");
}

#[tokio::test]
async fn a_reason_shown_elsewhere_is_pointed_to_not_repeated() {
    let mut props = reasoned(Some("Select a certificate first."));
    props.reason_elsewhere = Some("toolbar-why-not".into());
    let html = render::<ReasonedButton>(props).await;

    assert!(html.contains(r#"aria-describedby="toolbar-why-not""#), "{html}");
    assert!(!html.contains("Select a certificate first."), "said once, by the toolbar: {html}");
}

#[tokio::test]
async fn a_quiet_callout_leaves_the_announcing_to_its_live_region() {
    let mut props = callout(CalloutSeverity::Success, None);
    props.quiet = true;
    let html = render::<Callout>(props).await;

    assert!(!html.contains("role="), "{html}");
}
