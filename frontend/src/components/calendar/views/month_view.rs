use yew::{function_component, html, virtual_dom::VNode, Html, Properties, UseStateHandle};

use crate::core::{
    calendar_data::{get_todays_date, get_year_data},
    event_manager::EventDisplayManager,
    shared::Event,
    time::format_time,
};

use crate::components::modal::{modal_container::ModalContainer, modals::edit_event::EditEvent};

#[derive(Properties, PartialEq)]
struct DayParams {
    day_key: String,
    events: Vec<Event>,
    day: i32,
    active: bool,
    today: bool,
    show_right_edge: bool,
    show_bottom_edge: bool,
    modal: UseStateHandle<String>,
}

#[derive(Properties, PartialEq)]
pub struct MonthViewParams {
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    pub modal: UseStateHandle<String>,
    pub active_calendars: UseStateHandle<Vec<String>>,
}

#[function_component]
fn Day(props: &DayParams) -> Html {
    let inactive_style: String = "text-gray-500".to_string();
    let today_style: String =
        "rounded-full text-white bg-black dark:text-black dark:bg-white".to_string();

    let style = if props.today {
        today_style
    } else if !props.active {
        inactive_style
    } else {
        "".to_string()
    };

    let modal = props.modal.clone();
    let day = props.day;
    let day_key = props.day_key.clone();
    let events = props.events.clone();

    return html! {
        <div class={
            format!("flex flex-col border-2 border-transparent {} {}",
                if props.show_right_edge {"border-r-gray-800"} else {""},
                if props.show_bottom_edge {"border-b-gray-800"} else {""})
        }>
            <a class={format!("text-xl {} px-2 py-1 m-1", style)}>{ day }</a>
            <div class="flex flex-col w-full overflow-auto">
            {
                events.into_iter().map(move |event| {
                    let day_key = day_key.clone();
                    let modal = modal.clone();
                    let day_key_2 = day_key.clone();
                    let modal_2 = modal.clone();
                    html! {
                        <div
                            onclick={move |_| modal_2.set(day_key_2.clone())}
                            class="flex justify-between px-2 pb-1">
                            <a>{ event.name.clone() }</a>
                            <a>{ format_time(event.start.hour as i32, event.start.minute as i32) }</a>
                            {
                                if *modal == day_key {
                                    html! {
                                        <ModalContainer
                                            title={event.name.clone()}
                                            component={html! {<EditEvent event={event.clone()} day_key={day_key.clone()} />}}
                                            modal={modal.clone()} />
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    };
}

#[function_component]
pub fn MonthView(props: &MonthViewParams) -> Html {
    let mut day_components: Vec<VNode> = vec![];
    let display_manager = match EventDisplayManager::get_instance().lock() {
        Ok(manager) => manager,
        Err(_) => {
            return html! { <div>{"Error loading calendar(s), could not get display manager."}</div> };
        }
    };

    let current_month = get_year_data(*props.year)[*props.month as usize].clone();
    let (todays_month, todays_year, todays_day) = get_todays_date();

    let previous_month = if *props.month == 0 {
        get_year_data(*props.year - 1)[11].clone()
    } else {
        get_year_data(*props.year)[*props.month as usize - 1].clone()
    };

    let rows = if current_month.week_index + current_month.days_in_month > 7 * 5 {
        "grid-rows-6"
    } else {
        "grid-rows-5"
    };

    let days = if current_month.week_index + current_month.days_in_month > 7 * 5 {
        7 * 6
    } else {
        7 * 5
    };

    let current_month = get_year_data(*props.year)[*props.month as usize].clone();

    for i in (0..current_month.week_index).rev() {
        let is_today = if *props.month == 0 {
            todays_month == 11
                && todays_year == *props.year - 1
                && previous_month.days_in_month - i + 1 == todays_day
        } else {
            todays_month == *props.month - 1
                && todays_year == *props.year
                && previous_month.days_in_month - i + 1 == todays_day
        };

        for calendar in props.active_calendars.clone().iter() {
            let day_key = if *props.month == 0 {
                format!(
                    "{}-{}-{}-{}",
                    calendar,
                    *props.year - 1,
                    11,
                    previous_month.days_in_month - i + 1
                )
            } else {
                format!(
                    "{}-{}-{}-{}",
                    calendar,
                    *props.year,
                    *props.month - 1,
                    previous_month.days_in_month - i + 1
                )
            };

            day_components.push(html! {<Day
                day_key={day_key.clone()}
                events={display_manager.get_events_by_key(&day_key).clone()}
                day={previous_month.days_in_month - i}
                active={false}
                today={is_today}
                show_right_edge={true}
                show_bottom_edge={true}
                modal={props.modal.clone()}
            />});
        }

        if props.active_calendars.len() == 0 {
            day_components.push(html! {<Day
                day_key={"".to_string()}
                events={vec![]}
                day={previous_month.days_in_month - i}
                active={false}
                today={is_today}
                show_right_edge={true}
                show_bottom_edge={true}
                modal={props.modal.clone()}
            />});
        }
    }

    for i in 0..current_month.days_in_month {
        let is_today =
            todays_month == *props.month && todays_year == *props.year && i + 1 == todays_day;

        let show_right_edge = (current_month.week_index + i + 1) % 7 != 0;
        let show_bottom_edge = current_month.week_index + i < days - 7;

        for calendar in props.active_calendars.clone().iter() {
            let day_key = format!("{}-{}-{}-{}", calendar, *props.year, *props.month, i);

            day_components.push(html! {<Day
                day_key={day_key.clone()}
                events={display_manager.get_events_by_key(&day_key).clone()}
                day={i + 1}
                active={true}
                today={is_today}
                show_right_edge={show_right_edge}
                show_bottom_edge={show_bottom_edge}
                modal={props.modal.clone()}
            />});
        }

        if props.active_calendars.len() == 0 {
            day_components.push(html! {<Day
                day_key={"".to_string()}
                events={vec![]}
                day={i + 1}
                active={true}
                today={is_today}
                show_right_edge={show_right_edge}
                show_bottom_edge={show_bottom_edge}
                modal={props.modal.clone()}
            />});
        }
    }

    for i in 0..(days - current_month.week_index - current_month.days_in_month) {
        let is_today = if *props.month == 11 {
            todays_month == 0 && todays_year == *props.year + 1 && i + 1 == todays_day
        } else {
            todays_month == *props.month + 1 && todays_year == *props.year && i + 1 == todays_day
        };

        let show_right_edge =
            (current_month.week_index + current_month.days_in_month + i + 1) % 7 != 0;
        let show_bottom_edge =
            current_month.week_index + current_month.days_in_month + i < days - 7;

        for calendar in props.active_calendars.clone().iter() {
            let day_key = if *props.month == 11 {
                format!("{}-{}-{}-{}", calendar, *props.year + 1, 0, i + 1)
            } else {
                format!(
                    "{}-{}-{}-{}",
                    calendar,
                    *props.year,
                    *props.month + 1,
                    i + 1
                )
            };

            day_components.push(html! {<Day
                day_key={day_key.clone()}
                events={display_manager.get_events_by_key(&day_key).clone()}
                day={i + 1}
                active={false}
                today={is_today}
                show_right_edge={show_right_edge}
                show_bottom_edge={show_bottom_edge}
                modal={props.modal.clone()}
            />});
        }

        if props.active_calendars.len() == 0 {
            day_components.push(html! {<Day
                day_key={"".to_string()}
                events={vec![]}
                day={i + 1}
                active={false}
                today={is_today}
                show_right_edge={show_right_edge}
                show_bottom_edge={show_bottom_edge}
                modal={props.modal.clone()}
            />});
        }
    }

    return html! {
        <div class={format!("grid grid-cols-7 {} content-start w-full h-full border-2 border-black dark:border-transparent rounded-lg dark:bg-gray-900", rows)}>
            {day_components}
        </div>
    };
}
