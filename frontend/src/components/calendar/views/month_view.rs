use yew::{
    function_component, html, virtual_dom::VNode, Callback, Html, Properties, UseStateHandle,
};

use crate::core::{
    calendar_data::{get_todays_date, get_year_data},
    event_manager::EventDisplayManager,
    page_functions::calendar::ActiveCalendar,
    shared::Event,
    time::format_time,
};

use crate::components::modal::{modal_container::ModalContainer, modals::edit_event::EditEvent};

#[derive(Properties, PartialEq)]
struct DayParams {
    day_keys: Vec<String>,
    display_manager: EventDisplayManager,
    day: i32,
    active: bool,
    today: bool,
    show_right_edge: bool,
    show_bottom_edge: bool,
    modal: UseStateHandle<String>,
    token: String,
    refresh_data: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct MonthViewParams {
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    pub modal: UseStateHandle<String>,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub token: String,
    pub refresh_data: Callback<()>,
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
    let day_keys = props.day_keys.clone();

    let mut events_with_day_keys: (Vec<Event>, Vec<String>) = (vec![], vec![]);

    for day_key in day_keys.iter() {
        let got_events = props.display_manager.get_events_by_key(day_key).clone();

        for event in got_events.iter() {
            events_with_day_keys.0.push(event.clone());
            events_with_day_keys.1.push(day_key.clone());
        }
    }

    let mut event_elements: Vec<Html> = vec![];

    for i in 0..events_with_day_keys.0.len() {
        let event = events_with_day_keys.0[i].clone();
        let day_key = events_with_day_keys.1[i].clone();
        let day_key_2 = day_key.clone();

        let modal = modal.clone();
        let modal_2 = modal.clone();

        event_elements.push(html! {
            <div
                onclick={move |_| modal.set(day_key.clone())}
                class="flex justify-between px-2 pb-1">
                <a>{ event.name.clone() }</a>
                <a>{ format_time(event.start.hour as i32, event.start.minute as i32) }</a>
                {
                    if *modal_2 == day_key_2 {
                        html! {
                            <ModalContainer
                                title={event.name.clone()}
                                component={html! {<EditEvent event={event.clone()} day_key={day_key_2.clone()} token={props.token.clone()} modal={modal_2.clone()} refresh_data={props.refresh_data.clone()} />}}
                                modal={modal_2.clone()} />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        });
    }

    return html! {
        <div class={
            format!("flex flex-col border-2 border-transparent {} {}",
                if props.show_right_edge {"border-r-gray-800"} else {""},
                if props.show_bottom_edge {"border-b-gray-800"} else {""})
        }>
            <a class={format!("text-xl {} px-2 py-1 m-1", style)}>{ day }</a>
            <div class="flex flex-col w-full overflow-auto">
            {
                event_elements.into_iter().collect::<Html>()
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
                && previous_month.days_in_month - i == todays_day
        } else {
            todays_month == *props.month - 1
                && todays_year == *props.year
                && previous_month.days_in_month - i == todays_day
        };

        let mut day_keys: Vec<String> = vec![];

        for calendar in props.active_calendars.clone().iter() {
            let day_key = if *props.month == 0 {
                format!(
                    "{}-{}-{}-{}",
                    calendar.uuid,
                    *props.year - 1,
                    11,
                    previous_month.days_in_month - i - 1
                )
            } else {
                format!(
                    "{}-{}-{}-{}",
                    calendar.uuid,
                    *props.year,
                    *props.month - 1,
                    previous_month.days_in_month - i - 1
                )
            };

            day_keys.push(day_key);
        }

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={previous_month.days_in_month - i}
            active={false}
            today={is_today}
            show_right_edge={true}
            show_bottom_edge={true}
            modal={props.modal.clone()}
            token={props.token.clone()}
            refresh_data={props.refresh_data.clone()}
        />});
    }

    for i in 0..current_month.days_in_month {
        let is_today =
            todays_month == *props.month && todays_year == *props.year && i + 1 == todays_day;

        let show_right_edge = (current_month.week_index + i + 1) % 7 != 0;
        let show_bottom_edge = current_month.week_index + i < days - 7;

        let mut day_keys: Vec<String> = vec![];

        for calendar in props.active_calendars.clone().iter() {
            day_keys.push(format!(
                "{}-{}-{}-{}",
                calendar.uuid, *props.year, *props.month, i
            ));
        }

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={i + 1}
            active={true}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
            modal={props.modal.clone()}
            token={props.token.clone()}
            refresh_data={props.refresh_data.clone()}
        />});
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

        let mut day_keys: Vec<String> = vec![];

        for calendar in props.active_calendars.clone().iter() {
            let day_key = if *props.month == 11 {
                format!("{}-{}-{}-{}", calendar.uuid, *props.year + 1, 0, i + 1)
            } else {
                format!(
                    "{}-{}-{}-{}",
                    calendar.uuid,
                    *props.year,
                    *props.month + 1,
                    i + 1
                )
            };

            day_keys.push(day_key);
        }

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={i + 1}
            active={false}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
            modal={props.modal.clone()}
            token={props.token.clone()}
            refresh_data={props.refresh_data.clone()}
        />});
    }

    return html! {
        <div class={format!("grid grid-cols-7 {} content-start w-full h-full border-2 border-black dark:border-transparent rounded-lg dark:bg-gray-900", rows)}>
            {day_components}
        </div>
    };
}
