use web_sys::console;
use yew::{function_component, html, use_state, Html, Properties};

use crate::core::{
    shared::{Event, Time},
    time::{format_time, get_month_name, get_ordinal},
};

use crate::components::{
    main::{
        button::{Button, ButtonStyle},
        input_field::InputField,
    },
    modal::time_editor::{States, TimeEditor},
};

#[derive(Properties, PartialEq)]
pub struct EditEventParams {
    pub event: Event,
    pub day_key: String,
}

#[function_component]
pub fn EditEvent(props: &EditEventParams) -> Html {
    let event = props.event.clone();

    let editing = use_state(|| false);
    let open = use_state(|| "None".to_string());

    let event_clone = event.clone();
    let name = use_state(move || event_clone.name.clone());

    let start_states = States {
        day: use_state(move || event.start.day),
        month: use_state(move || event.start.month),
        year: use_state(move || event.start.year),
        hour: use_state(move || {
            if event.start.hour > 12 {
                event.start.hour - 12
            } else {
                event.start.hour
            }
        }),
        minute: use_state(move || event.start.minute),
        ampm: use_state(move || if event.start.hour > 12 { 1 } else { 0 }),
    };

    let end_states = States {
        day: use_state(move || event.end.day),
        month: use_state(move || event.end.month),
        year: use_state(move || event.end.year),
        hour: use_state(move || {
            if event.end.hour > 12 {
                event.end.hour - 12
            } else {
                event.end.hour
            }
        }),
        minute: use_state(move || event.end.minute),
        ampm: use_state(move || if event.end.hour > 12 { 1 } else { 0 }),
    };

    let start_day = format!(
        "{} of {}",
        get_ordinal((event.start.day + 1) as i32),
        get_month_name(event.start.month as i32)
    );

    let end_day = format!(
        "{} of {}",
        get_ordinal((event.end.day + 1) as i32),
        get_month_name(event.end.month as i32)
    );

    let start_time = format_time(event.start.hour as i32, event.start.minute as i32);
    let end_time = format_time(event.end.hour as i32, event.end.minute as i32);

    let start_states_clone = start_states.clone();
    let end_states_clone = end_states.clone();

    let name_clone = name.clone();
    let uuid_clone = event.uuid.clone();

    let handle_submit = move |_| {
        let new_event = Event {
            name: (*name_clone).clone(),
            uuid: uuid_clone.clone(),
            start: Time {
                day: *start_states_clone.day,
                month: *start_states_clone.month,
                year: *start_states_clone.year,
                hour: *start_states_clone.hour + (12 * *start_states_clone.ampm),
                minute: *start_states_clone.minute,
            },
            end: Time {
                day: *end_states_clone.day,
                month: *end_states_clone.month,
                year: *end_states_clone.year,
                hour: *end_states_clone.hour + (12 * *end_states_clone.ampm),
                minute: *end_states_clone.minute,
            },
        };

        console::log_1(&format!("{:?}", new_event).into());
    };

    html! {
        <div class="w-96">
            <div class="flex justify-between mt-4">
                <a>{ "Starts:" }</a>
                <div class="flex">{ format!("{}, {}", start_day, start_time) }</div>
            </div>
            <div class="flex justify-between">
                <a>{ "Ends:" }</a>
                <div class="flex">{ format!("{}, {}", end_day, end_time) }</div>
            </div>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            {
                if *editing {
                    html! {
                        <div>
                            <div class="flex justify-between items-center">
                                <a>{ "Name:" }</a>
                                <div class="flex justify-end w-48">
                                    <InputField<String> varient="text" value={ name } />
                                </div>
                            </div>
                            <TimeEditor id="Start" event={ event.clone() } open={ open.clone() } states={ start_states } />
                            <TimeEditor id="End" event={ event.clone() } open={ open.clone() } states={ end_states } />
                            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
                            <Button style={ ButtonStyle::Primary } width="" on_click={ handle_submit }>{ "Submit" }</Button>
                        </div>
                    }
                } else {
                    html! {
                        <Button style={ ButtonStyle::Primary } width="" on_click={ move |_| editing.set(true) }>{ "Edit" }</Button>
                    }
                }
            }
        </div>
    }
}
