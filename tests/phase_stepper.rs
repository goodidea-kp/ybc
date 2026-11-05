//! The phase stepper, rendered to HTML on the host (Bastion BASTION-697): an
//! ordered list whose current phase is `aria-current="step"`, whose states are
//! told by icon shape and words rather than colour, and whose phases carry test
//! ids derived from their keys.

use ybc::{Phase, PhaseState, PhaseStateWords, PhaseStepper, PhaseStepperProps};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render(props: PhaseStepperProps) -> String {
    LocalServerRenderer::<PhaseStepper>::with_props(props).hydratable(false).render().await
}

fn import_phases() -> Vec<Phase> {
    vec![
        Phase::new("schema", "Copy schema", PhaseState::Done),
        Phase::new("rows", "Copy rows", PhaseState::Current),
        Phase::new("verify", "Verify", PhaseState::Waiting).with_reason("Starts when every row is copied"),
        Phase::new("switch", "Switch over", PhaseState::Failed),
    ]
}

fn stepper(testid: Option<&str>) -> PhaseStepperProps {
    yew::props!(PhaseStepperProps {
        phases: import_phases(),
        aria_label: Some(AttrValue::from("Import progress")),
        testid: testid.map(AttrValue::from),
    })
}

fn phase_item<'html>(html: &'html str, testid: &str) -> &'html str {
    let marker = format!(r#"data-testid="{testid}""#);
    let start = html.find(&marker).unwrap_or_else(|| panic!("no {testid} in {html}"));
    let item_start = html[..start].rfind("<li").unwrap_or(start);
    let item_end = html[start..].find("</li>").map_or(html.len(), |end| start + end);
    &html[item_start..item_end]
}

#[tokio::test]
async fn the_phases_are_an_ordered_list_in_the_callers_order() {
    let html = render(stepper(Some("import-phases"))).await;

    assert!(html.starts_with("<ol"), "{html}");
    assert!(
        html.contains(r#"role="list""#),
        "list semantics survive list-style:none in Safari: {html}"
    );
    assert!(html.contains(r#"aria-label="Import progress""#), "{html}");
    assert!(html.contains(r#"data-testid="import-phases""#), "{html}");
    let positions: Vec<usize> = ["Copy schema", "Copy rows", "Verify", "Switch over"]
        .iter()
        .map(|label| html.find(label).unwrap_or_else(|| panic!("{label} missing: {html}")))
        .collect();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{html}");
}

#[tokio::test]
async fn only_the_current_phase_is_the_current_step() {
    let html = render(stepper(Some("import-phases"))).await;

    assert_eq!(html.matches(r#"aria-current="step""#).count(), 1, "{html}");
    assert!(phase_item(&html, "import-phases-rows").contains(r#"aria-current="step""#), "{html}");
}

#[tokio::test]
async fn every_state_has_its_own_icon_and_word_not_only_a_colour() {
    let html = render(stepper(Some("import-phases"))).await;

    for (testid, state) in [
        ("import-phases-schema", PhaseState::Done),
        ("import-phases-rows", PhaseState::Current),
        ("import-phases-verify", PhaseState::Waiting),
        ("import-phases-switch", PhaseState::Failed),
    ] {
        let item = phase_item(&html, testid);
        assert!(item.contains(&format!(r#"data-state="{}""#, state.code())), "{item}");
        assert!(item.contains(state.icon()), "{item}");
        assert!(item.contains(r#"aria-hidden="true""#), "the icon is decoration beside the word: {item}");
    }
    let words = PhaseStateWords::default();
    for word in [&words.done, &words.current, &words.waiting, &words.failed] {
        assert!(html.contains(&format!(r#"<span class="is-sr-only">: {word}</span>"#)), "{word}: {html}");
    }
}

#[tokio::test]
async fn the_state_words_come_from_the_caller() {
    let mut props = stepper(None);
    props.state_words = PhaseStateWords {
        done: "COMPLETED".into(),
        current: "RUNNING".into(),
        waiting: "PENDING".into(),
        failed: "FAILED".into(),
    };
    let html = render(props).await;

    assert!(html.contains(": RUNNING</span>"), "{html}");
    assert!(html.contains(": PENDING</span>"), "{html}");
    assert!(!html.contains("In progress"), "{html}");
}

#[tokio::test]
async fn a_reason_is_a_line_under_its_phase_only() {
    let html = render(stepper(Some("import-phases"))).await;

    assert!(
        phase_item(&html, "import-phases-verify").contains("Starts when every row is copied"),
        "{html}"
    );
    assert_eq!(html.matches("ybc-phase-reason").count(), 1, "{html}");
}

#[tokio::test]
async fn without_a_list_testid_each_phase_is_found_by_its_key() {
    let html = render(stepper(None)).await;

    assert!(html.contains(r#"data-testid="rows""#), "{html}");
    assert!(!html.contains(r#"data-testid="import-phases"#), "{html}");
}

#[tokio::test]
async fn phases_wrap_on_a_narrow_screen() {
    let html = render(stepper(None)).await;

    assert!(html.contains("flex-wrap:wrap"), "{html}");
    assert!(html.contains("flex:1 1 9rem"), "two phases fit a 400px row at most: {html}");
}
