use chrono::{Local, Timelike};
use wasm_bindgen_futures::spawn_local;

use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::{
    components::{
        main::{
            button::{Button, ButtonStyle},
            dropdown::DropDown,
            input_field::InputField,
            status::{Status, StatusCode, StatusObject},
        },
        modal::time_editor::TimeEditor,
    },
    core::{
        calendar_data::get_todays_date,
        page_functions::{
            calendar::ActiveCalendar,
            event::{create_event, use_get_states},
        },
        shared::{Event, Time},
    },
};

#[derive(Properties, PartialEq)]
pub struct CreateEventParams {
    pub token: String,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub modal: UseStateHandle<String>,
    pub refresh_data: Callback<()>,
}

#[function_component]
pub fn CreateEvent(props: &CreateEventParams) -> Html {
    let (todays_month, todays_year, todays_day) = get_todays_date();

    let now = Local::now();
    let hour = now.hour();

    let event = Event {
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

    let open = use_state(|| "None".to_string());
    let open_dropdown = use_state(|| "None".to_string());

    let active_calendar = props.active_calendars.clone();
    let active_calendar_index: UseStateHandle<usize> = use_state(|| 0);

    let mut active_calendar_options: Vec<Html> = vec![];

    for calendar in props.active_calendars.clone().iter() {
        active_calendar_options.push(html! {calendar.name.clone()});
    }

    let status = use_state(|| StatusObject {
        code: StatusCode::Ok,
        data: "".to_string(),
    });

    let name = use_state(|| event.name.clone());

    let start_states = use_get_states(event.start.clone());
    let end_states = use_get_states(event.end.clone());

    let token = props.token.clone();
    let status_clone = status.clone();

    let name_clone = name.clone();
    let modal = props.modal.clone();
    let calendar_id = props.active_calendars[0].uuid.clone();

    let start_states_clone = start_states.clone();
    let end_states_clone = end_states.clone();
    let refresh_data = props.refresh_data.clone();

    let handle_submit = move |_| {
        let name_clone = name_clone.clone();
        let calendar_id = calendar_id.clone();
        let modal = modal.clone();
        let refresh_data = refresh_data.clone();

        let start_states_clone = start_states_clone.clone();
        let end_states_clone = end_states_clone.clone();

        let token_clone = token.clone();
        let status_clone = status_clone.clone();

        status_clone.set(StatusObject {
            code: StatusCode::Loading,
            data: "Editing event...".to_string(),
        });

        spawn_local(async move {
            let code = create_event(
                name_clone.to_string(),
                start_states_clone.clone(),
                end_states_clone.clone(),
                calendar_id.clone(),
                token_clone.to_string(),
            )
            .await;

            if code == 200 {
                status_clone.set(StatusObject {
                    code: StatusCode::Ok,
                    data: "Event edited successfully".to_string(),
                });

                modal.set("None".to_string());
                refresh_data.emit(());
            } else {
                status_clone.set(StatusObject {
                    code: StatusCode::Error,
                    data: format!("Error editing event: {}", code),
                });
            }
        });
    };

    let active_calendar_index_clone = active_calendar_index.clone();
    let open_dropdown_clone = open_dropdown.clone();

    let handle_calendar_change = move |index: usize| {
        active_calendar_index_clone.set(index);
        open_dropdown_clone.set("None".to_string());
    };

    html! {
        <div class="w-96 pt-1">
            <div class="flex justify-between items-center py-1">
                <a>{ "Calendar:" }</a>
                <DropDown
                    open={ open_dropdown.clone() }
                    id="Selected_Calendar"
                    minimal={ false }
                    element={ html! { <a>{active_calendar[*active_calendar_index.clone()].name.clone()}</a> }}
                    options={active_calendar_options.clone()}
                    return_index={handle_calendar_change}
                />
            </div>
            <div class="flex justify-between items-center">
                <a>{ "Name:" }</a>
                <div class="flex justify-end w-48">
                    <InputField<String> varient="text" value={ name.clone() } />
                </div>
            </div>
            <TimeEditor id="Start" event={ event.clone() } open={ open.clone() } states={ start_states.clone() } />
            <TimeEditor id="End" event={ event.clone() } open={ open.clone() } states={ end_states.clone() } />
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <Button style={ ButtonStyle::Primary } width="" on_click={ handle_submit }>{ "Submit" }</Button>
            <Status status={status.clone()} />
        </div>
    }
}
