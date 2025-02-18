use sqlx::Error;

use crate::core::init_db::EventTable;
use crate::services::database::Database;

use super::super::shared::Time;

use super::super::get::events::get_event;

pub async fn update_event(
    uuid: &String,
    name: &String,
    start: &Time,
    end: &Time,
    database: &Database,
) -> Result<(), Error> {
    let event_from_db: Vec<EventTable> = get_event(uuid, database).await?;

    database
        .write_db(
            "UPDATE times SET year = $1, month = $2, day = $3, hour = $4, minute = $5 WHERE uuid = $6",
            vec![start.year.to_string(), start.month.to_string(), start.day.to_string(), start.hour.to_string(), start.minute.to_string(), event_from_db[0].start_id.clone()],
        )
        .await?;

    database
        .write_db(
            "UPDATE times SET year = $1, month = $2, day = $3, hour = $4, minute = $5 WHERE uuid = $6",
            vec![end.year.to_string(), end.month.to_string(), end.day.to_string(), end.hour.to_string(), end.minute.to_string(), event_from_db[0].end_id.clone()]
        )
        .await?;

    database
        .write_db(
            "UPDATE events SET name = $1 WHERE uuid = $2",
            vec![name.to_string(), uuid.to_string()],
        )
        .await?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::calendar::{
        create::event::tests::get_database_with_filled_calendar, get::calendars::get_calendars,
        parse_calendar_data::parse_calendar,
    };
    use crate::core::init_db::CalendarTable;

    #[tokio::test]
    async fn test_update_event() {
        let database: Database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        let parsed_calendar = match parse_calendar(&calendars_from_db[0], &database).await {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to get calendars. {}", e)
            }
        };

        let start = Time {
            year: 2026,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
        };

        let end = Time {
            year: 2026,
            month: 1,
            day: 12,
            hour: 0,
            minute: 0,
        };

        match update_event(
            &parsed_calendar.events[0].uuid,
            &"New years day".to_string(),
            &start,
            &end,
            &database,
        )
        .await
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to get calendars. {}", e)
            }
        };
    }
}
