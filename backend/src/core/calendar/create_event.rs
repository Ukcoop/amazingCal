use sqlx::Error;
use uuid::Uuid;

use crate::services::database::Database;

use super::shared::Time;

pub async fn create_event(calendar_id: String, name: String, start: Time, end: Time, database: &Database) -> Result<(), Error> {
    let event_id = Uuid::new_v4().to_string();
    let start_id = Uuid::new_v4().to_string();
    let end_id = Uuid::new_v4().to_string();

    database
        .write_db(
            "INSERT INTO times (event_id, year, month, day, hour, minute) VALUES ($1, $2, $3, $4, $5, $6)",
            vec![event_id.clone(), start.year.to_string(), start.month.to_string(), start.day.to_string(), start.hour.to_string(), start.minute.to_string()],
        )
    .await?;

    database
        .write_db(
            "INSERT INTO times (event_id, year, month, day, hour, minute) VALUES ($1, $2, $3, $4, $5, $6)",
            vec![event_id.clone(), end.year.to_string(), end.month.to_string(), end.day.to_string(), end.hour.to_string(), end.minute.to_string()],
        )
    .await?;

    database
        .write_db(
            "INSERT INTO events (calendar_id, start_id, end_id, name) VALUES ($1, $2, $3, $4)",
            vec![calendar_id, start_id, end_id, name]
        )
    .await?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::init_db::init_db;

    #[tokio::test]
    async fn test_create_event() {
        // needs get_calendar()
    }
}
