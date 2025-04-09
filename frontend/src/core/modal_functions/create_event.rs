use chrono::{Local, Timelike};
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Callback, Html, UseStateHandle};

use crate::components::{
    main::status::{StatusCode, StatusObject},
    modal::time_editor::{States, StatesContainer},
};

use crate::core::{
    api::post, calendar_data::get_todays_date, page_functions::calendar::ActiveCalendar,
    shared::Time,
};

#[derive(Serialize, PartialEq, Clone)]
pub struct CreateNewEvent {
    pub calendar_id: String,
    pub name: String,
    pub start: Time,
    pub end: Time,
}

pub fn get_calendar_options(active_calendars: UseStateHandle<Vec<ActiveCalendar>>) -> Vec<Html> {
    let mut active_calendar_options: Vec<Html> = vec![];

    for calendar in active_calendars.iter() {
        active_calendar_options.push(html! {calendar.name.clone()});
    }

    return active_calendar_options;
}

pub fn new_event() -> CreateNewEvent {
    let (todays_month, todays_year, todays_day) = get_todays_date();
    let now = Local::now();
    let hour = now.hour();

    return CreateNewEvent {
        calendar_id: "".to_string(),
        name: "Event".to_string(),
        start: Time {
            year: todays_year as u16,
            month: todays_month as u8,
            day: (todays_day - 1) as u8,
            hour: hour as u8,
            minute: 0,
        },
        end: Time {
            year: todays_year as u16,
            month: todays_month as u8,
            day: (todays_day - 1) as u8,
            hour: (hour + 1) as u8,
            minute: 0,
        },
    };
}

pub async fn create_event(
    name: String,
    start: States,
    end: States,
    calendar_id: String,
    token: String,
) -> u16 {
    let new_event = CreateNewEvent {
        calendar_id,
        name,
        start: Time {
            day: *start.day,
            month: *start.month,
            year: *start.year,
            hour: *start.hour + (12 * *start.ampm),
            minute: *start.minute,
        },
        end: Time {
            day: *end.day,
            month: *end.month,
            year: *end.year,
            hour: *end.hour + (12 * *end.ampm),
            minute: *end.minute,
        },
    };

    return post::<CreateNewEvent>("http://localhost:3080/api/create/event", &token, &new_event)
        .await;
}

pub fn handle_submit(
    name: String,
    calendar_id: String,
    modal: UseStateHandle<String>,
    refresh_data: Callback<()>,
    states: StatesContainer,
    token: String,
    status: UseStateHandle<StatusObject>,
) {
    status.set(StatusObject {
        code: StatusCode::Loading,
        data: "Editing event...".to_string(),
    });

    spawn_local(async move {
        let code = create_event(
            name.to_string(),
            states.start.clone(),
            states.end.clone(),
            calendar_id.clone(),
            token.to_string(),
        )
        .await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Ok,
                data: "Event created successfully".to_string(),
            });

            modal.set("None".to_string());
            refresh_data.emit(());
        } else {
            status.set(StatusObject {
                code: StatusCode::Error,
                data: format!("Error editing event: {}", code),
            });
        }
    });
}

pub fn handle_calendar_change(
    index: usize,
    active_calendar_index: UseStateHandle<usize>,
    open_dropdown: UseStateHandle<String>,
) {
    active_calendar_index.set(index);
    open_dropdown.set("None".to_string());
}
