use sqlx::Error;

use crate::services::database::Database;

pub async fn init_db(database: &Database) -> Result<(), Error> {
    let calendar_table = "
        CREATE TABLE IF NOT EXISTS calendars (
            userId TEXT NOT NULL,
            uuid TEXT NOT NULL,
            name TEXT NOT NULL
        )";

    let event_table = "
        CREATE TABLE IF NOT EXISTS events (
            calendarId TEXT NOT NULL,
            uuid TEXT NOT NULL,
            name TEXT NOT NULL
        )";

    let time_table = "
        CREATE TABLE IF NOT EXISTS times (
            eventId TEXT NOT NULL,
            year NUMBER NOT NULL,
            month NUMBER NOT NULL,
            day NUMBER NOT NULL,
            hour NUMBER NOT NULL,
            minute NUMBER NOT NULL
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
            Ok(_) => {},
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }
    }
}
