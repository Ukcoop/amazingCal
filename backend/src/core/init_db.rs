use sqlx::{Error, FromRow};

use crate::services::database::Database;

#[derive(FromRow, Clone, Debug, PartialEq)]
pub struct CalendarTable {
    pub user_id: String,
    pub uuid: String,
    pub name: String,
}

#[derive(FromRow, Clone, Debug, PartialEq)]
pub struct EventTable {
    pub calendar_id: String,
    pub name: String,
    pub start_id: String,
    pub end_id: String,
}

#[derive(FromRow, Clone, Debug, PartialEq)]
pub struct TimeTable {
    pub event_id: String,
    pub year: i16,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    pub minute: i16,
}

pub async fn init_db(database: &Database) -> Result<(), Error> {
    let calendar_table = "
        CREATE TABLE IF NOT EXISTS calendars (
            user_id TEXT NOT NULL,
            uuid TEXT NOT NULL,
            name TEXT NOT NULL
        )";

    let event_table = "
        CREATE TABLE IF NOT EXISTS events (
            calendar_id TEXT NOT NULL,
            start_id TEXT NOT NULL,
            end_id TEXT NOT NULL,
            name TEXT NOT NULL
        )";

    let time_table = "
        CREATE TABLE IF NOT EXISTS times (
            event_id TEXT NOT NULL,
            year SMALLINT NOT NULL,
            month SMALLINT NOT NULL,
            day SMALLINT NOT NULL,
            hour SMALLINT NOT NULL,
            minute SMALLINT NOT NULL
        )";

    database.write_db(calendar_table, vec![]).await?;
    database.write_db(event_table, vec![]).await?;
    database.write_db(time_table, vec![]).await?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::convert_sqlx_error;

    #[tokio::test]
    async fn test_init_db() {
        let database = match convert_sqlx_error(Database::new_db(true, "".to_string()).await) {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }
    }
}
