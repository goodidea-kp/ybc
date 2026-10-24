//! What the [`super::calendar::Calendar`] picker shows inside its dialog: the
//! month's heading between Previous and Next, a grid of day buttons, and Close.
//! It holds no state; the Calendar owns which day has focus.

use super::calendar_month::{Day, WEEKDAY_NAMES, month_weeks, moved_by_key};
use yew::prelude::*;

/// The attribute each day button carries, with the day as `yyyy-mm-dd`.
pub(super) const DAY_ATTRIBUTE: &str = "data-ybc-day";
/// Letters a weekday's column header shows; its full name is the `abbr`.
const WEEKDAY_LETTERS: usize = 2;

#[derive(Clone, PartialEq, Properties)]
pub(super) struct CalendarPopupProps {
    /// The Calendar's id; the heading is `{id}-picker-title`.
    pub id: AttrValue,
    /// The day the grid's one tab stop sits on; the grid shows its month.
    pub focused: Day,
    #[prop_or_default]
    pub selected: Option<Day>,
    #[prop_or_default]
    pub today: Option<Day>,
    pub on_pick: Callback<Day>,
    /// A new focused day, and whether focus moves onto it (keys) or stays
    /// where it is (the month buttons).
    pub on_focus: Callback<(Day, bool)>,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
}

pub(super) fn title_id(id: &str) -> String {
    format!("{id}-picker-title")
}

fn suffixed(testid: &Option<AttrValue>, suffix: &str) -> Option<String> {
    testid.as_ref().map(|testid| format!("{testid}-{suffix}"))
}

#[function_component(CalendarPopup)]
pub(super) fn calendar_popup(props: &CalendarPopupProps) -> Html {
    let title_id = title_id(&props.id);
    html! {
        <div class="ybc-calendar-body">
            { month_head(props, &title_id) }
            <table role="grid" aria-labelledby={title_id} onkeydown={grid_keys(props.on_focus.clone())}>
                <thead>
                    <tr>
                        { for WEEKDAY_NAMES.iter().map(|name| html! {
                            <th key={*name} scope="col" abbr={*name}>{ &name[..WEEKDAY_LETTERS] }</th>
                        }) }
                    </tr>
                </thead>
                <tbody>
                    { for month_weeks(props.focused).into_iter().enumerate().map(|(index, week)| html! {
                        <tr key={index}>{ for week.into_iter().enumerate().map(|(column, cell)| day_cell(props, column, cell)) }</tr>
                    }) }
                </tbody>
            </table>
            <div class="ybc-calendar-foot">
                <button type="button" class="button is-small" data-testid={suffixed(&props.testid, "close")}
                        onclick={props.on_close.reform(|_| ())}>{ "Close" }</button>
            </div>
        </div>
    }
}

fn month_head(props: &CalendarPopupProps, title_id: &str) -> Html {
    let previous = props.focused.plus_months(-1);
    let next = props.focused.plus_months(1);
    let month_button = |target: Day, label: &str, icon: &str, suffix: &str| {
        let on_focus = props.on_focus.clone();
        html! {
            <button type="button" class="button is-small" aria-label={format!("{label}, {}", target.month_title())}
                    data-testid={suffixed(&props.testid, suffix)} onclick={move |_| on_focus.emit((target, false))}>
                <span class="icon"><i class={classes!("fas", icon.to_owned())} aria-hidden="true"></i></span>
            </button>
        }
    };
    html! {
        <div class="ybc-calendar-head">
            { month_button(previous, "Previous month", "fa-chevron-left", "previous") }
            <p id={title_id.to_owned()} class="ybc-calendar-title" aria-live="polite">{ props.focused.month_title() }</p>
            { month_button(next, "Next month", "fa-chevron-right", "next") }
        </div>
    }
}

fn day_cell(props: &CalendarPopupProps, column: usize, cell: Option<Day>) -> Html {
    let Some(day) = cell else {
        return html! { <td key={format!("blank-{column}")}></td> };
    };
    let is_selected = props.selected == Some(day);
    let is_focused = props.focused == day;
    let class = classes!("button", "ybc-calendar-day", is_selected.then_some("is-link"));
    let on_pick = props.on_pick.clone();
    html! {
        <td key={day.iso()} aria-selected={is_selected.to_string()}>
            <button type="button" {class} tabindex={if is_focused { "0" } else { "-1" }}
                    data-ybc-day={day.iso()} aria-label={day.spoken()}
                    aria-current={(props.today == Some(day)).then_some("date")}
                    onclick={move |_| on_pick.emit(day)}>
                { day.day }
            </button>
        </td>
    }
}

/// Arrow keys, Home/End and PageUp/PageDown move the focused day. The day comes
/// from the button the key was pressed on, found from `event.target()`: under
/// Yew's delegated listeners `current_target()` is the app's root.
fn grid_keys(on_focus: Callback<(Day, bool)>) -> Callback<KeyboardEvent> {
    Callback::from(move |event: KeyboardEvent| {
        let Some(day) = day_of_target(&event) else {
            return;
        };
        if let Some(moved) = moved_by_key(day, &event.key()) {
            event.prevent_default();
            on_focus.emit((moved, true));
        }
    })
}

fn day_of_target(event: &KeyboardEvent) -> Option<Day> {
    use wasm_bindgen::JsCast;
    let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
    let button = target.closest(&format!("[{DAY_ATTRIBUTE}]")).ok()??;
    Day::parse_iso(&button.get_attribute(DAY_ATTRIBUTE)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::LocalServerRenderer;

    fn day(iso: &str) -> Day {
        Day::parse_iso(iso).unwrap_or_else(|| panic!("{iso} is a real date"))
    }

    async fn render(selected: Option<&str>, today: Option<&str>) -> String {
        let props = CalendarPopupProps {
            id: "start".into(),
            focused: day("2026-10-24"),
            selected: selected.map(day),
            today: today.map(day),
            on_pick: Callback::noop(),
            on_focus: Callback::noop(),
            on_close: Callback::noop(),
            testid: Some("start-picker".into()),
        };
        LocalServerRenderer::<CalendarPopup>::with_props(props).hydratable(false).render().await
    }

    #[tokio::test]
    async fn the_grid_is_named_by_its_month_and_every_day_by_its_full_date() {
        let html = render(None, None).await;
        assert!(
            html.contains(r#"role="grid""#) && html.contains(r#"aria-labelledby="start-picker-title""#),
            "{html}"
        );
        assert!(html.contains(r#"id="start-picker-title""#) && html.contains("October 2026"), "{html}");
        assert!(html.contains(r#"aria-label="Saturday 24 October 2026""#), "{html}");
        assert!(html.contains(r#"abbr="Monday""#) && html.contains(">Mo<"), "{html}");
        assert_eq!(html.matches(DAY_ATTRIBUTE).count(), 31, "one button per day of October");
    }

    #[tokio::test]
    async fn only_the_focused_day_is_a_tab_stop() {
        let html = render(None, None).await;
        assert_eq!(html.matches(r#"tabindex="0""#).count(), 1, "{html}");
        assert!(html.contains(r#"tabindex="0" data-ybc-day="2026-10-24""#), "{html}");
    }

    #[tokio::test]
    async fn the_month_buttons_name_the_month_they_go_to() {
        let html = render(None, None).await;
        assert!(html.contains(r#"aria-label="Previous month, September 2026""#), "{html}");
        assert!(html.contains(r#"aria-label="Next month, November 2026""#), "{html}");
        assert!(html.contains(r#"data-testid="start-picker-close""#) && html.contains(">Close<"), "{html}");
    }

    #[tokio::test]
    async fn the_picked_day_is_selected_and_today_is_the_current_date() {
        let html = render(Some("2026-10-20"), Some("2026-10-24")).await;
        assert_eq!(html.matches(r#"aria-selected="true""#).count(), 1, "{html}");
        assert_eq!(html.matches("is-link").count(), 1, "{html}");
        assert!(html.contains(r#"aria-current="date""#), "{html}");
    }
}
