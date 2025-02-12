use sqlx::Error;
use uuid::Uuid;

use crate::services::database::Database;

use super::shared::Time;

pub async fn create_event(
    calendar_id: String,
    name: String,
    start: Time,
    end: Time,
    database: &Database,
) -> Result<(), Error> {
    let event_id = Uuid::new_v4().to_string();
    let start_id = Uuid::new_v4().to_string();
    let end_id = Uuid::new_v4().to_string();

    database
        .write_db(
            "INSERT INTO times (uuid, year, month, day, hour, minute) VALUES ($1, $2, $3, $4, $5, $6)",
            vec![start_id.clone(), start.year.to_string(), start.month.to_string(), start.day.to_string(), start.hour.to_string(), start.minute.to_string()],
        )
    .await?;

    database
        .write_db(
            "INSERT INTO times (uuid, year, month, day, hour, minute) VALUES ($1, $2, $3, $4, $5, $6)",
            vec![end_id.clone(), end.year.to_string(), end.month.to_string(), end.day.to_string(), end.hour.to_string(), end.minute.to_string()],
        )
    .await?;

    database
        .write_db(
            "INSERT INTO events (calendar_id, uuid, start_id, end_id, name) VALUES ($1, $2, $3, $4, $5)",
            vec![calendar_id, event_id, start_id, end_id, name],
        )
        .await?;

    return Ok(());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::core::calendar::{
        create_calendar::create_calendar, get_calendars::get_calendars, shared::Time,
    };
    use crate::core::init_db::{tests::get_testable_db, CalendarTable};

    pub async fn get_database_with_filled_calendar() -> Database {
        let database: Database = get_testable_db().await;

        match create_calendar("test_user".to_string(), "test".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add calendar. {}", e)
            }
        }

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars("test_user", &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e)
                }
            };

        let start = Time {
            year: 2025,
            month: 12,
            day: 31,
            hour: 0,
            minute: 0,
        };

        let end = Time {
            year: 2026,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
        };

        match create_event(
            calendars_from_db[0].uuid.clone(),
            "New years eve".to_string(),
            start,
            end,
            &database,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add event. {}", e)
            }
        }

        return database;
    }

    #[tokio::test]
    async fn test_create_event() {
        get_database_with_filled_calendar().await;
    }
}
