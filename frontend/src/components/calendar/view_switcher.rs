use yew::{function_component, html, Html, Properties, UseStateHandle};

use super::views::{month_view::MonthView, schedule_view::ScheduleView};

#[derive(Properties, PartialEq)]
pub struct CalendarViewParams {
    pub view: UseStateHandle<String>,
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    // needed when modals are implemented: pub modal: UseStateHandle<String>,
}

#[function_component]
pub fn CalendarView(props: &CalendarViewParams) -> Html {
    return match props.view.as_str() {
        "Month" => html! {<MonthView month={props.month.clone()} year={props.year.clone()} />},
        "Schedule" => html! {<ScheduleView />},
        _ => {
            html! {<div class="flex w-full h-full items-center justify-center rounded-lg dark:bg-gray-900">{"Calendar view."}</div>}
        }
    };
}
