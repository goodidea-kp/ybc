pub(super) fn to_native(value: &str, format: &str, kind: &str) -> String {
    let value = value.trim();
    if kind == "time" {
        return if valid_time(value) { value.to_owned() } else { String::new() };
    }
    let (date, time) = value.split_once(['T', ' ']).unwrap_or((value, ""));
    let parts: Vec<_> = date.split(if is_us(format) { '/' } else { '-' }).collect();
    if parts.len() != 3 {
        return String::new();
    }
    let (year, month, day) = if is_us(format) {
        (parts[2], parts[0], parts[1])
    } else {
        (parts[0], parts[1], parts[2])
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 || ![year, month, day].iter().all(|s| s.bytes().all(|b| b.is_ascii_digit())) {
        return String::new();
    }
    let (Ok(y), Ok(m), Ok(d)) = (year.parse::<u32>(), month.parse::<u32>(), day.parse::<u32>()) else {
        return String::new();
    };
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = match m {
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => 0,
    };
    if y == 0 || d == 0 || d > days {
        return String::new();
    }
    let native = format!("{year}-{month}-{day}");
    if kind == "datetime-local" {
        if valid_time(time) { format!("{native}T{time}") } else { String::new() }
    } else {
        native
    }
}

pub(super) fn from_native(value: &str, format: &str, kind: &str) -> String {
    let native = to_native(value, "yyyy-MM-dd", kind);
    if native.is_empty() || kind == "time" {
        return native;
    }
    let (date, time) = native.split_once('T').unwrap_or((&native, ""));
    let date = if is_us(format) {
        format!("{}/{}/{}", &date[5..7], &date[8..10], &date[..4])
    } else {
        date.to_owned()
    };
    if time.is_empty() { date } else { format!("{date} {time}") }
}

fn is_us(format: &str) -> bool {
    format.trim().eq_ignore_ascii_case("mm/dd/yyyy")
}

fn valid_time(value: &str) -> bool {
    let parts: Vec<_> = value.split(':').collect();
    (parts.len() == 2 || parts.len() == 3)
        && parts.iter().enumerate().all(|(i, part)| {
            part.len() == 2 && part.bytes().all(|b| b.is_ascii_digit()) && part.parse::<u32>().is_ok_and(|n| n < if i == 0 { 24 } else { 60 })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_dates_round_trip() {
        assert_eq!(to_native("02/29/2024", "mm/dd/yyyy", "date"), "2024-02-29");
        assert_eq!(from_native("2024-02-29", "mm/dd/yyyy", "date"), "02/29/2024");
    }

    #[test]
    fn datetime_and_time_round_trip() {
        assert_eq!(to_native("12/31/2030 01:02", "mm/dd/yyyy", "datetime-local"), "2030-12-31T01:02");
        assert_eq!(from_native("2030-12-31T01:02", "mm/dd/yyyy", "datetime-local"), "12/31/2030 01:02");
        assert_eq!(to_native("01:02", "", "time"), "01:02");
    }

    #[test]
    fn empty_invalid_and_unsupported_values_are_safe() {
        for value in ["", " ", "02/29/2023", "13/01/2024", "04/31/2024", "garbage"] {
            assert_eq!(to_native(value, "mm/dd/yyyy", "date"), "");
        }
        assert_eq!(to_native("2024-02-29", "unsupported", "date"), "2024-02-29");
        assert_eq!(from_native("", "mm/dd/yyyy", "date"), "");
        assert_eq!(to_native("2030-01-01", "yyyy-MM-dd", "date"), "2030-01-01");
    }

    #[test]
    fn leap_centuries_and_malformed_values_are_validated() {
        assert_eq!(to_native("02/29/2000", "MM/dd/yyyy", "date"), "2000-02-29");
        for value in ["02/29/1900", "00/01/2024", "01/00/2024", "01/01/0000", "éé/01/2024"] {
            assert_eq!(to_native(value, "mm/dd/yyyy", "date"), "");
        }
        for value in ["24:00", "12:60", "1:02", "12:30:60", "not a time"] {
            assert_eq!(to_native(value, "", "time"), "");
        }
        assert_eq!(to_native("2030-01-01 23:59:59", "", "datetime-local"), "2030-01-01T23:59:59");
        assert_eq!(from_native("invalid", "mm/dd/yyyy", "date"), "");
    }
}
