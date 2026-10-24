//! The Calendar field rendered to HTML on the host (Bastion BASTION-539): the
//! typable input a keyboard uses, and the named button that opens the picker.
//! What the picker does in a browser is in `tests/browser/calendar.test.mjs`.

use ybc::{CALENDAR_CLASS, Calendar, CalendarProps, TestAttr};
use yew::LocalServerRenderer;
use yew::prelude::*;

fn field() -> CalendarProps {
    CalendarProps {
        id: "start".into(),
        date_format: AttrValue::default(),
        time_format: AttrValue::default(),
        date: Some("2026-10-24".into()),
        on_date_changed: Callback::noop(),
        class: vec!["is-danger".into()],
        test_attr: Some(TestAttr::test_id("start-date")),
        calendar_type: AttrValue::default(),
        display_mode: AttrValue::default(),
        disabled: false,
        aria_label: Some("From date".into()),
        aria_describedby: Some("start-preview start-problem".into()),
        invalid: true,
        picker_label: None,
    }
}

async fn render(props: CalendarProps) -> String {
    LocalServerRenderer::<Calendar>::with_props(props).hydratable(false).render().await
}

#[tokio::test]
async fn the_field_stays_a_native_date_input_a_keyboard_types_into() {
    let html = render(field()).await;
    assert!(html.contains(r#"type="date""#) && html.contains(r#"value="2026-10-24""#), "{html}");
    assert!(html.contains(r#"id="start""#) && html.contains(r#"data-testid="start-date""#), "{html}");
    assert!(!html.contains("readonly"), "the field is typable: {html}");
    assert!(html.contains(r#"aria-label="From date""#), "{html}");
    assert!(html.contains(r#"aria-describedby="start-preview start-problem""#), "{html}");
    assert!(html.contains(r#"aria-invalid="true""#) && html.contains("is-danger"), "{html}");
    assert!(html.contains(CALENDAR_CLASS), "{html}");
}

#[tokio::test]
async fn the_picker_button_is_named_and_says_it_opens_a_dialog() {
    let html = render(field()).await;
    assert!(html.contains(r#"aria-label="Choose From date from a calendar""#), "{html}");
    assert!(
        html.contains(r#"aria-haspopup="dialog""#) && html.contains(r#"aria-expanded="false""#),
        "{html}"
    );
    assert!(html.contains(r#"aria-controls="start-picker-dialog""#), "{html}");
    assert!(html.contains(r#"data-testid="start-date-picker""#), "{html}");
    assert!(html.contains(r#"data-testid="start-date-picker-dialog""#), "{html}");
    assert!(html.contains("fa-calendar-days") && html.contains(r#"aria-hidden="true""#), "{html}");
}

#[tokio::test]
async fn the_picker_starts_closed_and_empty() {
    let html = render(field()).await;
    assert!(html.contains("<dialog") && !html.contains("<dialog open"), "{html}");
    assert!(!html.contains("data-ybc-day"), "no days until it opens: {html}");
}

#[tokio::test]
async fn a_given_picker_label_wins_and_the_default_names_a_date() {
    let named = render(CalendarProps {
        picker_label: Some("Choose the first day".into()),
        ..field()
    })
    .await;
    assert!(named.contains(r#"aria-label="Choose the first day""#), "{named}");
    let unnamed = render(CalendarProps { aria_label: None, ..field() }).await;
    assert!(unnamed.contains(r#"aria-label="Choose a date from a calendar""#), "{unnamed}");
}

#[tokio::test]
async fn a_disabled_field_disables_its_picker_too() {
    let html = render(CalendarProps { disabled: true, ..field() }).await;
    assert_eq!(html.matches(r#"disabled="disabled""#).count(), 2, "the input and the button: {html}");
}

#[tokio::test]
async fn the_button_takes_the_size_of_the_input() {
    let html = render(CalendarProps {
        class: vec!["is-small".into()],
        ..field()
    })
    .await;
    assert!(html.contains(r#"class="button is-small""#), "{html}");
}

#[tokio::test]
async fn us_dates_reach_the_input_in_its_own_shape() {
    let html = render(CalendarProps {
        date_format: "mm/dd/yyyy".into(),
        date: Some("10/24/2026".into()),
        ..field()
    })
    .await;
    assert!(html.contains(r#"value="2026-10-24""#), "{html}");
}

#[tokio::test]
async fn time_and_datetime_fields_have_no_picker() {
    for calendar_type in ["time", "datetime"] {
        let html = render(CalendarProps {
            calendar_type: calendar_type.into(),
            date: None,
            ..field()
        })
        .await;
        assert!(!html.contains("<dialog") && !html.contains("-picker"), "{calendar_type}: {html}");
    }
}
