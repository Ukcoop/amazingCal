use sqlx::Error;

use crate::services::database::Database;

use crate::core::calendar::get_events::get_events;

pub async fn delete_calendar(uuid: &String, database: &Database) -> Result<(), Error> {
    let events = get_events(uuid, database).await?;

    for event in events {
        database
            .write_db("DELETE FROM events WHERE uuid = $1", vec![event.uuid])
            .await?;
    }

    database
        .write_db(
            "DELETE FROM calendars WHERE uuid = $1",
            vec![uuid.to_string()],
        )
        .await?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::calendar::{
        create_event::tests::get_database_with_filled_calendar, get_calendars::get_calendars,
    };
    use crate::core::init_db::CalendarTable;

    #[tokio::test]
    async fn test_delete_calendar() {
        let database: Database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        match delete_calendar(&calendars_from_db[0].uuid, &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to delete calendar. {}", e)
            }
        }
    }
}
