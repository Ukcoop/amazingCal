use chrono::{Local, Timelike};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Callback, Html, UseStateHandle};

use crate::{
    components::{
        main::status::{StatusCode, StatusObject},
        modal::time_editor::States,
    },
    core::{
        calendar_data::get_todays_date,
        page_functions::calendar::ActiveCalendar,
        shared::{Event, Time},
    },
};

use super::edit_event::create_event;

pub fn get_calendar_options(active_calendars: UseStateHandle<Vec<ActiveCalendar>>) -> Vec<Html> {
    let mut active_calendar_options: Vec<Html> = vec![];

    for calendar in active_calendars.iter() {
        active_calendar_options.push(html! {calendar.name.clone()});
    }

    return active_calendar_options;
}

pub fn new_event() -> Event {
    let (todays_month, todays_year, todays_day) = get_todays_date();
    let now = Local::now();
    let hour = now.hour();

    return Event {
        name: "Event".to_string(),
        uuid: "".to_string(),
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

pub fn handle_submit(
    name: String,
    calendar_id: String,
    modal: UseStateHandle<String>,
    refresh_data: Callback<()>,
    start_states: States,
    end_states: States,
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
            start_states.clone(),
            end_states.clone(),
            calendar_id.clone(),
            token.to_string(),
        )
        .await;

        if code == 200 {
            status.set(StatusObject {
                code: StatusCode::Ok,
                data: "Event edited successfully".to_string(),
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
