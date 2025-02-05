use sqlx::Error;

use crate::services::database::Database;

use crate::core::init_db::{CalendarTable, EventTable, TimeTable};
use crate::core::calendar::create_calendar::create_calendar;

use super::shared::{Calendar, Event, Time, UserData};

pub async fn get_user_data(uuid: &String, database: &Database) -> Result<UserData, Error> {
    let mut calendars: Vec<Calendar> = Vec::new();

    let mut calendars_from_db = database
        .read_db::<CalendarTable>(
            format!(
                "SELECT user_id, uuid, name FROM calendars WHERE user_id == \"{}\"",
                uuid
            )
            .as_str(),
        )
        .await?;

    if calendars_from_db.is_empty() {
        create_calendar(uuid.clone(), "default".to_string(), &database).await?;
        calendars_from_db = database
            .read_db::<CalendarTable>(
                format!(
                    "SELECT user_id, uuid, name FROM calendars WHERE user_id == \"{}\"",
                    uuid
                )
                .as_str(),
            )
            .await?;
    }

    for calendar in calendars_from_db {
        let mut new_calendar = Calendar {
            name: calendar.name,
            events: Vec::new(),
        };

        let events_from_db = database.read_db::<EventTable>(
            format!("SELECT calendar_id, name, start_id, end_id FROM events WHERE calendar_id == \"{}\"", calendar.uuid).as_str()
        ).await?;

        for event in events_from_db {
            let start_time = database.read_db::<TimeTable>(
                format!("SELECT event_id, year, month, day, hour, minute FROM times WHERE event_id == \"{}\"", event.start_id).as_str()
            ).await?;

            let end_time = database.read_db::<TimeTable>(
                format!("SELECT event_id, year, month, day, hour, minute FROM times WHERE event_id == \"{}\"", event.end_id).as_str()
            ).await?;

            new_calendar.events.push(Event {
                name: event.name,
                start: Time {
                    year: start_time[0].year,
                    month: start_time[0].month,
                    day: start_time[0].day,
                    hour: start_time[0].hour,
                    minute: start_time[0].minute,
                },
                end: Time {
                    year: end_time[0].year,
                    month: end_time[0].month,
                    day: end_time[0].day,
                    hour: end_time[0].hour,
                    minute: end_time[0].minute,
                },
            });
        }

        calendars.push(new_calendar);
    }

    return Ok(UserData { calendars });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::init_db::init_db;

    #[tokio::test]
    async fn test_get_user_data() {
        let database: Database = match Database::new_db(true, "".to_string()).await {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e)
            }
        };

        match init_db(&database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to initialize database. {}", e);
            }
        }

        match get_user_data(&"test_user".to_string(), &database).await {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: failed to get user data. {}", e);
            }
        }
    }
}
