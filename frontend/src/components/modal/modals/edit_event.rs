use yew::{function_component, html, Html, Properties};

use crate::core::{
    shared::Event,
    time::{format_time, get_month_name, get_ordinal},
};

#[derive(Properties, PartialEq)]
pub struct EditEventParams {
    pub event: Event,
    pub day_key: String,
}

#[function_component]
pub fn EditEvent(props: &EditEventParams) -> Html {
    let start_day = format!(
        "{} of {}",
        get_ordinal((props.event.start.day + 1) as i32),
        get_month_name(props.event.start.month as i32)
    );

    let end_day = format!(
        "{} of {}",
        get_ordinal((props.event.end.day + 1) as i32),
        get_month_name(props.event.end.month as i32)
    );

    let start_time = format_time(
        props.event.start.hour as i32,
        props.event.start.minute as i32,
    );

    let end_time = format_time(props.event.end.hour as i32, props.event.end.minute as i32);

    html! {
        <div class="w-96">
            <div class="flex justify-between mt-4">
                <a>{"Starts:"}</a>
                <div class="flex">{format!("{}, {}", start_day, start_time)}</div>
            </div>
            <div class="flex justify-between">
                <a>{"Ends:"}</a>
                <div class="flex">{format!("{}, {}", end_day, end_time)}</div>
            </div>
        </div>
    }
}
