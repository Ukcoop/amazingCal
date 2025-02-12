use sqlx::Error;

use crate::services::database::Database;

use crate::core::calendar::get_events::get_event;
use crate::core::init_db::EventTable;

pub async fn delete_event(uuid: &String, database: &Database) -> Result<(), Error> {
    let event_from_db: Vec<EventTable> = get_event(uuid, database).await?;

    database
        .write_db(
            "DELETE FROM times WHERE uuid = $1",
            vec![event_from_db[0].start_id.clone()],
        )
        .await?;

    database
        .write_db(
            "DELETE FROM times WHERE uuid = $1",
            vec![event_from_db[0].end_id.clone()],
        )
        .await?;

    database
        .write_db("DELETE FROM events WHERE uuid = $1", vec![uuid.to_string()])
        .await?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::calendar::{
        create_event::tests::get_database_with_filled_calendar, get_calendars::get_calendars,
        parse_calendar_data::parse_calendar,
    };
    use crate::core::init_db::CalendarTable;

    #[tokio::test]
    async fn test_delete_event() {
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

        match delete_event(&parsed_calendar.events[0].uuid, &database).await {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to delete event. {}", e)
            }
        }
    }
}
