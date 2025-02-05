use sqlx::Error;
 
use crate::services::database::Database;
use crate::core::init_db::{CalendarTable, EventTable, TimeTable};

use super::shared::{Calendar, Event, Time, UserData};

use super::get_calendars::get_calendars;
use super::create_calendar::create_calendar;

pub async fn get_user_data(uuid: &String, database: &Database) -> Result<UserData, Error> {
    let mut calendars: Vec<Calendar> = Vec::new();

    let mut calendars_from_db: Vec<CalendarTable> = get_calendars(uuid, &database).await?;

    if calendars_from_db.is_empty() {
        create_calendar(uuid.clone(), "default".to_string(), &database).await?;
        calendars_from_db = get_calendars(uuid, &database).await?;
    }

    for calendar in calendars_from_db {
        let mut new_calendar = Calendar {
            name: calendar.name,
            events: Vec::new(),
        };

        let events_from_db = database.read_db::<EventTable>(
            "SELECT calendar_id, name, start_id, end_id FROM events WHERE calendar_id == $1",
            vec![calendar.uuid]
        ).await?;

        for event in events_from_db {
            let start_time = database.read_db::<TimeTable>(
                "SELECT event_id, year, month, day, hour, minute FROM times WHERE event_id == $1",
                vec![event.start_id]
            ).await?;

            let end_time = database.read_db::<TimeTable>(
                "SELECT event_id, year, month, day, hour, minute FROM times WHERE event_id == $1",
                vec![event.end_id]
            ).await?;

            new_calendar.events.push(Event {
                name: event.name,
                start: Time {
                    year: start_time[0].year.parse::<u16>().unwrap_or(0),
                    month: start_time[0].month.parse::<u8>().unwrap_or(0),
                    day: start_time[0].day.parse::<u8>().unwrap_or(0),
                    hour: start_time[0].hour.parse::<u8>().unwrap_or(0),
                    minute: start_time[0].minute.parse::<u8>().unwrap_or(0),
                },
                end: Time {
                    year: end_time[0].year.parse::<u16>().unwrap_or(0),
                    month: end_time[0].month.parse::<u8>().unwrap_or(0),
                    day: end_time[0].day.parse::<u8>().unwrap_or(0),
                    hour: end_time[0].hour.parse::<u8>().unwrap_or(0),
                    minute: end_time[0].minute.parse::<u8>().unwrap_or(0)
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
