use sqlx::Error;

use crate::services::database::Database;

pub async fn update_calendar(
    uuid: &String,
    name: &String,
    database: &Database,
) -> Result<(), Error> {
    database
        .write_db(
            "UPDATE calendars SET name = $1 WHERE uuid = $2",
            vec![name.to_string(), uuid.to_string()],
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
    async fn test_update_calendar() {
        let database: Database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        match update_calendar(&calendars_from_db[0].uuid, &"testy".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to update calendar. {}", e)
            }
        }
    }
}
