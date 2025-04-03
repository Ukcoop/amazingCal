use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::core::page_functions::calendar::ActiveCalendar;

use super::views::{month_view::MonthView, schedule_view::ScheduleView};

#[derive(Properties, PartialEq)]
pub struct CalendarViewParams {
    pub view: UseStateHandle<String>,
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    pub modal: UseStateHandle<String>,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub token: String,
    pub refresh_data: Callback<()>,
}

#[function_component]
pub fn CalendarView(props: &CalendarViewParams) -> Html {
    return match props.view.as_str() {
        "Month" => {
            html! {<MonthView month={props.month.clone()} year={props.year.clone()} modal={props.modal.clone()} active_calendars={props.active_calendars.clone()} token={props.token.clone()} refresh_data={props.refresh_data.clone()} />}
        }
        "Schedule" => html! {<ScheduleView />},
        _ => {
            html! {<div class="flex w-full h-full items-center justify-center rounded-lg dark:bg-gray-900">{"Calendar view."}</div>}
        }
    };
}
