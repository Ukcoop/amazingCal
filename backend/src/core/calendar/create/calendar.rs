use sqlx::Error;
use uuid::Uuid;

use crate::services::database::Database;

pub async fn create_calendar(
    user_id: &String,
    name: &String,
    database: &Database,
) -> Result<(), Error> {
    database
        .write_db(
            "INSERT INTO calendars (user_id, uuid, name) VALUES ($1, $2, $3)",
            vec![
                user_id.to_string(),
                Uuid::new_v4().to_string(),
                name.to_string(),
            ],
        )
        .await?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::init_db::tests::get_testable_db;

    #[tokio::test]
    async fn test_create_calendar() {
        let database: Database = get_testable_db().await;

        match create_calendar(&"test_user".to_string(), &"test".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to add calendar. {}", e)
            }
        }
    }
}
