use chrono::{DateTime, Datelike, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct MonthData {
    pub week_index: i32,
    pub days_in_month: i32,
}

fn get_days_in_month(date: DateTime<chrono::Local>) -> i32 {
    let next_month = date
        .with_day(1)
        .and_then(|d| d.checked_add_months(chrono::Months::new(1)))
        .and_then(|d| d.checked_sub_days(chrono::Days::new(1)))
        .unwrap_or(date);
    next_month.day() as i32
}

pub fn get_year_data(year: i32) -> Vec<MonthData> {
    let mut year_data = Vec::with_capacity(12);

    for month in 0..12 {
        let date = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .unwrap_or_else(|| {
                NaiveDate::from_ymd_opt(2000, 1, 1)
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .unwrap_or_else(|| Local::now().naive_local())
            });

        let local_date = date
            .and_local_timezone(Local)
            .single()
            .unwrap_or_else(Local::now);

        let days_in_month = get_days_in_month(local_date);

        let month_data = MonthData {
            week_index: date.weekday().num_days_from_sunday() as i32,
            days_in_month,
        };

        year_data.push(month_data);
    }

    year_data
}

pub fn get_todays_date() -> (i32, i32, i32) {
    let now = Local::now();

    let month = now.month() - 1;
    let year = now.year();
    let day = now.day();

    (month as i32, year, day as i32)
}
