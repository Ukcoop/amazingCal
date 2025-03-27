use chrono::NaiveDate;

pub fn get_month_name(month_number: i32) -> String {
    let month = month_number + 1;
    match NaiveDate::from_ymd_opt(2023, month as u32, 1) {
        Some(date) => date.format("%B").to_string(),
        None => "Invalid month".to_string(),
    }
}

pub fn get_ordinal(n: i32) -> String {
    let suffix = match n % 10 {
        1 if n % 100 != 11 => "st",
        2 if n % 100 != 12 => "nd",
        3 if n % 100 != 13 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

pub fn format_time(hours: i32, minutes: i32) -> String {
    let am_or_pm = if hours >= 12 { "pm" } else { "am" };
    let display_hours = hours % 12;
    let display_hours = if display_hours == 0 {
        12
    } else {
        display_hours
    };
    let str_minutes = format!("{:02}", minutes);
    format!("{}:{} {}", display_hours, str_minutes, am_or_pm)
}
