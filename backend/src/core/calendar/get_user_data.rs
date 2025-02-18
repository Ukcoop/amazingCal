use sqlx::Error;

use crate::core::init_db::CalendarTable;
use crate::services::database::Database;

use super::shared::{Calendar, UserData};

use super::parse_calendar_data::parse_calendar;

use super::create_calendar::create_calendar;
use super::get_calendars::get_calendars;

pub async fn get_user_data(uuid: &str, database: &Database) -> Result<UserData, Error> {
    let mut calendars: Vec<Calendar> = Vec::new();

    let mut calendars_from_db: Vec<CalendarTable> = get_calendars(uuid, database).await?;

    if calendars_from_db.is_empty() {
        create_calendar(&uuid.to_string(), &"default".to_string(), database).await?;
        calendars_from_db = get_calendars(uuid, database).await?;
    }

    for calendar in calendars_from_db {
        calendars.push(parse_calendar(&calendar, database).await?);
    }

    return Ok(UserData { calendars });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::init_db::tests::get_testable_db;

    #[tokio::test]
    async fn test_get_user_data() {
        let database: Database = get_testable_db().await;

        match get_user_data(&"test_user".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to get user data. {}", e);
            }
        }
    }
}
