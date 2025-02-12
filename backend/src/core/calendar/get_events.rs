use sqlx::Error;

use crate::core::init_db::EventTable;
use crate::services::database::Database;

pub async fn get_events(calendar_id: &str, database: &Database) -> Result<Vec<EventTable>, Error> {
    return database
        .read_db::<EventTable>(
            "SELECT calendar_id, uuid, start_id, end_id, name FROM events WHERE calendar_id == $1",
            vec![calendar_id.to_owned()],
        )
        .await;
}

pub async fn get_event(event_id: &str, database: &Database) -> Result<Vec<EventTable>, Error> {
    return database
        .read_db::<EventTable>(
            "SELECT calendar_id, uuid, start_id, end_id, name FROM events WHERE uuid == $1",
            vec![event_id.to_owned()],
        )
        .await;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::core::calendar::get_calendars::get_calendars;
    use crate::core::init_db::CalendarTable;

    use crate::core::calendar::create_event::tests::get_database_with_filled_calendar;

    #[tokio::test]
    async fn test_get_event() {
        let database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        match get_event(&calendars_from_db[0].uuid, &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to get event. {}", e)
            }
        }
    }
}
