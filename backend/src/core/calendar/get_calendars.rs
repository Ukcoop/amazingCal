use sqlx::Error;

use crate::core::init_db::CalendarTable;
use crate::services::database::Database;

pub async fn get_calendars(user_id: &String, database: &Database) -> Result<Vec<CalendarTable>, Error> {
    return Ok(database
        .read_db::<CalendarTable>(
            "SELECT user_id, uuid, name FROM calendars WHERE user_id == $1",
            vec![user_id.clone()]
        )
    .await?);
}
