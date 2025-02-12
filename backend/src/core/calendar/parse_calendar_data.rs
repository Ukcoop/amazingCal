use sqlx::Error;

use crate::services::database::Database;

use super::shared::{Calendar, Event, Time};
use crate::core::init_db::{CalendarTable, EventTable, TimeTable};

pub async fn parse_event(event_from_db: &EventTable, database: &Database) -> Result<Event, Error> {
    let start_time = database
        .read_db::<TimeTable>(
            "SELECT uuid, year, month, day, hour, minute FROM times WHERE uuid == $1",
            vec![event_from_db.start_id.clone()],
        )
        .await?;

    let end_time = database
        .read_db::<TimeTable>(
            "SELECT uuid, year, month, day, hour, minute FROM times WHERE uuid == $1",
            vec![event_from_db.end_id.clone()],
        )
        .await?;

    return Ok(Event {
        name: event_from_db.name.clone(),
        uuid: event_from_db.uuid.clone(),
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
            minute: end_time[0].minute.parse::<u8>().unwrap_or(0),
        },
    });
}

pub async fn parse_calendar(
    calendar_from_db: &CalendarTable,
    database: &Database,
) -> Result<Calendar, Error> {
    let mut calendar = Calendar {
        name: calendar_from_db.name.clone(),
        uuid: calendar_from_db.uuid.clone(),
        events: Vec::new(),
    };

    let events_from_db = database
        .read_db::<EventTable>(
            "SELECT calendar_id, uuid, name, start_id, end_id FROM events WHERE calendar_id == $1",
            vec![calendar_from_db.uuid.clone()],
        )
        .await?;

    for event in events_from_db {
        calendar.events.push(parse_event(&event, database).await?);
    }

    return Ok(calendar);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::calendar::create_event::tests::get_database_with_filled_calendar;
    use crate::core::calendar::get_calendars::get_calendars;

    #[tokio::test]
    async fn test_parse_event() {
        let database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars(&"test_user".to_string(), &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e);
                }
            };

        let events_from_db = match database
            .read_db::<EventTable>(
                "SELECT calendar_id, uuid, name, start_id, end_id FROM events WHERE calendar_id == $1",
                vec![calendars_from_db[0].uuid.clone()],
            )
            .await
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Error: failed to get events. {}", e);
            }
        };

        for event in events_from_db {
            match parse_event(&event, &database).await {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error: failed to parse event. {}", e);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_parse_calendar() {
        let database = get_database_with_filled_calendar().await;

        let calendars_from_db: Vec<CalendarTable> =
            match get_calendars(&"test_user".to_string(), &database).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error: failed to get calendars. {}", e);
                }
            };

        for calendar in calendars_from_db {
            match parse_calendar(&calendar, &database).await {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error: failed to parse calendar. {}", e);
                }
            }
        }
    }
}
