//! The colour scheme choice and its toggle, rendered to HTML on the host
//! (Bastion BASTION-102). Outside a browser the stored choice is System.

use ybc::theme::{CONTRAST_ATTRIBUTE, THEME_STORAGE_KEY, stored};
use ybc::{Theme, ThemeToggle, ThemeToggleProps};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render_toggle(testid: Option<&str>) -> String {
    let props = ThemeToggleProps {
        label: "Colour scheme".into(),
        onchange: None,
        classes: Classes::new(),
        testid: testid.map(AttrValue::from),
    };
    LocalServerRenderer::<ThemeToggle>::with_props(props).hydratable(false).render().await
}

#[test]
fn system_removes_the_attribute_and_the_others_name_bulmas_scheme() {
    assert_eq!(Theme::System.attribute_value(), None);
    assert_eq!(Theme::Light.attribute_value(), Some("light"));
    assert_eq!(Theme::Dark.attribute_value(), Some("dark"));
}

#[test]
fn high_contrast_is_bulmas_light_scheme_plus_the_contrast_attribute() {
    assert_eq!(Theme::HighContrast.attribute_value(), Some("light"));
    assert_eq!(Theme::HighContrast.contrast_value(), Some("more"));
    assert_eq!(CONTRAST_ATTRIBUTE, "data-contrast");
    for theme in [Theme::System, Theme::Light, Theme::Dark] {
        assert_eq!(theme.contrast_value(), None, "{theme:?} removes data-contrast");
    }
}

#[test]
fn high_contrast_is_stored_under_its_own_name_not_as_light() {
    assert_eq!(Theme::HighContrast.storage_value(), "high-contrast");
    assert_eq!(Theme::from_storage_value("high-contrast"), Theme::HighContrast);
    assert_eq!(Theme::from_storage_value("light"), Theme::Light);
}

#[test]
fn every_choice_survives_a_round_trip_through_storage() {
    for theme in Theme::ALL {
        assert_eq!(Theme::from_storage_value(theme.storage_value()), theme);
    }
}

#[test]
fn an_unknown_or_empty_stored_value_means_follow_the_system() {
    for value in ["", "Dark", "High-Contrast", "highcontrast", "null", " light"] {
        assert_eq!(Theme::from_storage_value(value), Theme::System, "{value:?}");
    }
}

#[test]
fn outside_a_browser_nothing_is_stored_and_the_default_is_system() {
    assert_eq!(stored(), Theme::System);
    assert_eq!(Theme::default(), Theme::System);
    assert_eq!(THEME_STORAGE_KEY, "ybc.theme");
}

#[tokio::test]
async fn the_toggle_is_a_named_group_of_four_buttons_with_visible_text() {
    let html = render_toggle(None).await;

    assert!(html.contains(r#"role="group""#), "{html}");
    assert!(html.contains(r#"aria-label="Colour scheme""#), "{html}");
    assert_eq!(html.matches(r#"<button type="button""#).count(), 4, "{html}");
    for theme in Theme::ALL {
        assert!(html.contains(&format!("<span>{}</span>", theme.label())), "{html}");
    }
}

#[tokio::test]
async fn only_the_current_choice_is_pressed() {
    let html = render_toggle(None).await;

    assert_eq!(html.matches(r#"aria-pressed="true""#).count(), 1, "{html}");
    assert_eq!(html.matches(r#"aria-pressed="false""#).count(), 3, "{html}");
    let pressed = html.find(r#"aria-pressed="true""#).expect("one pressed button");
    let system_label = html.find("<span>System</span>").expect("a System button");
    let light_label = html.find("<span>Light</span>").expect("a Light button");
    assert!(pressed < system_label && system_label < light_label, "System is the pressed one: {html}");
}

#[tokio::test]
async fn icons_are_hidden_from_a_screen_reader() {
    let html = render_toggle(None).await;

    assert_eq!(html.matches(r#"aria-hidden="true""#).count(), 4, "{html}");
}

#[tokio::test]
async fn each_button_gets_a_testid_derived_from_the_group() {
    let with_testid = render_toggle(Some("theme")).await;
    let without = render_toggle(None).await;

    for value in ["theme-system", "theme-light", "theme-dark", "theme-high-contrast"] {
        assert!(with_testid.contains(&format!(r#"data-testid="{value}""#)), "{with_testid}");
    }
    assert!(!without.contains("data-testid"), "{without}");
}
