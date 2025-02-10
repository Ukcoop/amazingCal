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
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
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
            year TEXT NOT NULL,
            month TEXT NOT NULL,
            day TEXT NOT NULL,
            hour TEXT NOT NULL,
            minute TEXT NOT NULL
        )";

    database.write_db(calendar_table, vec![]).await?;
    database.write_db(event_table, vec![]).await?;
    database.write_db(time_table, vec![]).await?;

    return Ok(());
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::services::database::convert_sqlx_error;

    pub async fn get_testable_db() -> Database {
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

        return database;
    }

    #[tokio::test]
    async fn test_init_db() {
        get_testable_db().await;
    }
}
