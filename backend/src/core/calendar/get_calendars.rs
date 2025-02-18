use sqlx::Error;

use crate::core::init_db::CalendarTable;
use crate::services::database::Database;

pub async fn get_calendars(
    user_id: &str,
    database: &Database,
) -> Result<Vec<CalendarTable>, Error> {
    return database
        .read_db::<CalendarTable>(
            "SELECT user_id, uuid, name FROM calendars WHERE user_id == $1",
            vec![user_id.to_owned()],
        )
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::calendar::create_calendar::create_calendar;
    use crate::core::init_db::tests::get_testable_db;

    #[tokio::test]
    async fn test_get_calendars() {
        let database: Database = get_testable_db().await;

        match create_calendar(&"test_user".to_string(), &"test".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add calendar. {}", e)
            }
        }

        match get_calendars(&"test_user".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to get calendars. {}", e)
            }
        }
    }
}
