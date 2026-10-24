//! The date arithmetic behind the [`super::calendar::Calendar`] picker: one day,
//! the weeks of its month, and where each key moves it. Pure, so the host tests
//! cover it without a browser.

/// Days in one week; the picker's weeks start on Sunday, as a US calendar does.
pub(super) const DAYS_PER_WEEK: u32 = 7;
const MONTHS_PER_YEAR: u32 = 12;
const FIRST_YEAR: u32 = 1;
const LAST_YEAR: u32 = 9999;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Sunday first, as the column headers read them.
pub(super) const WEEKDAY_NAMES: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

/// One calendar day. Always a real date between years 1 and 9999.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Day {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Day {
    /// A real date, or `None`.
    pub fn new(year: u32, month: u32, day: u32) -> Option<Day> {
        let in_range = (FIRST_YEAR..=LAST_YEAR).contains(&year) && (1..=MONTHS_PER_YEAR).contains(&month);
        (in_range && day >= 1 && day <= days_in_month(year, month)).then_some(Day { year, month, day })
    }

    /// Reads `yyyy-mm-dd`, the native date input's own shape.
    pub fn parse_iso(value: &str) -> Option<Day> {
        let mut parts = value.trim().splitn(3, '-');
        let (year, month, day) = (parts.next()?, parts.next()?, parts.next()?);
        if year.len() != 4 || month.len() != 2 || day.len() != 2 {
            return None;
        }
        Day::new(year.parse().ok()?, month.parse().ok()?, day.parse().ok()?)
    }

    /// `yyyy-mm-dd`.
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// 0 for Sunday to 6 for Saturday (Sakamoto's method).
    pub fn weekday(self) -> u32 {
        const MONTH_OFFSETS: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let year = if self.month < 3 { self.year - 1 } else { self.year };
        (year + year / 4 - year / 100 + year / 400 + MONTH_OFFSETS[(self.month - 1) as usize] + self.day) % DAYS_PER_WEEK
    }

    /// "Friday 24 October 2026": what a screen reader says for the day's button.
    pub fn spoken(self) -> String {
        format!(
            "{} {} {} {}",
            WEEKDAY_NAMES[self.weekday() as usize],
            self.day,
            month_name(self.month),
            self.year
        )
    }

    /// "October 2026": the picker's heading for this day's month.
    pub fn month_title(self) -> String {
        format!("{} {}", month_name(self.month), self.year)
    }

    /// The day `offset` days away; the same day at either end of the supported years.
    pub fn plus_days(self, offset: i64) -> Day {
        let target = self.ordinal() + offset;
        Day::from_ordinal(target).unwrap_or(self)
    }

    /// The same day of the month `offset` months away, clamped to that month's
    /// length (31 January plus one month is 28 or 29 February).
    pub fn plus_months(self, offset: i64) -> Day {
        let months = i64::from(self.year) * i64::from(MONTHS_PER_YEAR) + i64::from(self.month - 1) + offset;
        let (Ok(year), Ok(month_index)) = (u32::try_from(months.div_euclid(12)), u32::try_from(months.rem_euclid(12))) else {
            return self;
        };
        let month = month_index + 1;
        Day::new(year, month, self.day.min(days_in_month(year, month))).unwrap_or(self)
    }

    /// The first day of this day's month.
    pub fn first_of_month(self) -> Day {
        Day { day: 1, ..self }
    }

    /// Days since 0001-01-01 (day 0).
    fn ordinal(self) -> i64 {
        let previous_years = i64::from(self.year - 1);
        let leap_days = previous_years / 4 - previous_years / 100 + previous_years / 400;
        let days_before_month: i64 = (1..self.month).map(|month| i64::from(days_in_month(self.year, month))).sum();
        previous_years * 365 + leap_days + days_before_month + i64::from(self.day - 1)
    }

    fn from_ordinal(ordinal: i64) -> Option<Day> {
        if ordinal < 0 {
            return None;
        }
        let mut year = FIRST_YEAR + u32::try_from(ordinal / 366).ok()?;
        while year < LAST_YEAR
            && (Day {
                year: year + 1,
                month: 1,
                day: 1,
            })
            .ordinal()
                <= ordinal
        {
            year += 1;
        }
        let mut remaining = ordinal - Day::new(year, 1, 1)?.ordinal();
        let mut month = 1;
        while month < MONTHS_PER_YEAR && remaining >= i64::from(days_in_month(year, month)) {
            remaining -= i64::from(days_in_month(year, month));
            month += 1;
        }
        Day::new(year, month, u32::try_from(remaining).ok()? + 1)
    }
}

fn month_name(month: u32) -> &'static str {
    MONTH_NAMES.get((month.saturating_sub(1)) as usize).copied().unwrap_or_default()
}

fn is_leap(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

/// 28 to 31; 0 for a month that does not exist.
pub(super) fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0,
    }
}

/// The weeks of `day`'s month, Sunday first; `None` pads the cells before the
/// 1st and after the last day.
pub(super) fn month_weeks(day: Day) -> Vec<[Option<Day>; 7]> {
    let first = day.first_of_month();
    let leading_blanks = first.weekday();
    let length = days_in_month(first.year, first.month);
    let cells = (leading_blanks + length).div_ceil(DAYS_PER_WEEK) * DAYS_PER_WEEK;
    let days: Vec<Option<Day>> = (0..cells)
        .map(|cell| {
            cell.checked_sub(leading_blanks)
                .and_then(|index| Day::new(first.year, first.month, index + 1))
        })
        .collect();
    days.chunks(DAYS_PER_WEEK as usize)
        .map(|week| [week[0], week[1], week[2], week[3], week[4], week[5], week[6]])
        .collect()
}

/// Where a key moves the focused day in the grid, as the WAI-ARIA date picker
/// dialog pattern has it; `None` for a key the grid leaves alone.
pub(super) fn moved_by_key(day: Day, key: &str) -> Option<Day> {
    let weekday = i64::from(day.weekday());
    let week = i64::from(DAYS_PER_WEEK);
    match key {
        "ArrowLeft" => Some(day.plus_days(-1)),
        "ArrowRight" => Some(day.plus_days(1)),
        "ArrowUp" => Some(day.plus_days(-week)),
        "ArrowDown" => Some(day.plus_days(week)),
        "Home" => Some(day.plus_days(-weekday)),
        "End" => Some(day.plus_days(week - 1 - weekday)),
        "PageUp" => Some(day.plus_months(-1)),
        "PageDown" => Some(day.plus_months(1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(iso: &str) -> Day {
        Day::parse_iso(iso).unwrap_or_else(|| panic!("{iso} is a real date"))
    }

    #[test]
    fn only_real_dates_in_the_input_shape_parse() {
        assert_eq!(day("2024-02-29").iso(), "2024-02-29");
        for value in [
            "2023-02-29",
            "2024-13-01",
            "2024-04-31",
            "24-01-01",
            "2024-1-01",
            "",
            "2024/01/01",
            "0000-01-01",
        ] {
            assert_eq!(Day::parse_iso(value), None, "{value:?}");
        }
    }

    #[test]
    fn weekdays_count_from_sunday() {
        assert_eq!(day("2026-10-24").weekday(), 6, "a Saturday");
        assert_eq!(day("2026-10-18").weekday(), 0, "a Sunday");
        assert_eq!(day("2026-10-19").weekday(), 1, "a Monday");
        assert_eq!(day("2000-01-01").weekday(), 6);
        assert_eq!(day("2024-02-29").spoken(), "Thursday 29 February 2024");
        assert_eq!(day("2026-10-24").month_title(), "October 2026");
    }

    #[test]
    fn days_and_months_cross_their_boundaries() {
        assert_eq!(day("2026-12-31").plus_days(1), day("2027-01-01"));
        assert_eq!(day("2024-03-01").plus_days(-1), day("2024-02-29"));
        assert_eq!(day("2026-01-31").plus_months(1), day("2026-02-28"));
        assert_eq!(day("2026-01-15").plus_months(-1), day("2025-12-15"));
        assert_eq!(day("9999-12-31").plus_days(1), day("9999-12-31"), "stays at the end");
        assert_eq!(day("0001-01-01").plus_days(-1), day("0001-01-01"), "stays at the start");
    }

    #[test]
    fn a_month_is_whole_weeks_sunday_first() {
        let weeks = month_weeks(day("2026-10-24"));
        assert_eq!(weeks.len(), 5);
        assert_eq!(weeks[0][..4], [None, None, None, None], "1 October 2026 is a Thursday");
        assert_eq!(weeks[0][4], Some(day("2026-10-01")));
        assert_eq!(weeks[4][6], Some(day("2026-10-31")), "a Saturday ends the last week");
        let february = month_weeks(day("2026-02-10"));
        assert_eq!(february.len(), 4, "February 2026 starts on a Sunday and fills four weeks");
        assert_eq!(february[0][0], Some(day("2026-02-01")));
    }

    #[test]
    fn keys_move_the_focused_day_as_the_aria_pattern_says() {
        let saturday = day("2026-10-24");
        assert_eq!(moved_by_key(saturday, "ArrowRight"), Some(day("2026-10-25")));
        assert_eq!(moved_by_key(saturday, "ArrowLeft"), Some(day("2026-10-23")));
        assert_eq!(moved_by_key(saturday, "ArrowDown"), Some(day("2026-10-31")));
        assert_eq!(moved_by_key(saturday, "ArrowUp"), Some(day("2026-10-17")));
        assert_eq!(moved_by_key(saturday, "Home"), Some(day("2026-10-18")), "back to Sunday");
        assert_eq!(moved_by_key(saturday, "End"), Some(saturday), "Saturday ends the week");
        assert_eq!(moved_by_key(saturday, "PageDown"), Some(day("2026-11-24")));
        assert_eq!(moved_by_key(saturday, "PageUp"), Some(day("2026-09-24")));
        assert_eq!(moved_by_key(saturday, "Enter"), None, "Enter is the button's own");
        assert_eq!(moved_by_key(saturday, "a"), None);
    }
}
