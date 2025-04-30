use chrono::{Local, Timelike};
use yew::{
    function_component, html, use_state, virtual_dom::VNode, Callback, Html, MouseEvent,
    Properties, UseStateHandle,
};

use crate::core::{
    calendar_data::{get_todays_date, get_year_data},
    event_manager::EventDisplayManager,
    modal_functions::create_event::CreateNewEvent,
    page_functions::calendar::ActiveCalendar,
    shared::{Event, Time},
    time::format_time,
};

use crate::components::{
    main::right_click_menu::RightClickMenu,
    modal::{modal_container::ModalContainer, modals::edit_event::EditEvent},
};

#[derive(PartialEq, Clone)]
pub struct ContextMenuDeps {
    pub context_menu: UseStateHandle<String>,
    pub modal: UseStateHandle<String>,
    pub event: UseStateHandle<CreateNewEvent>,
}

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
    context_menu_deps: ContextMenuDeps,
    new_event: CreateNewEvent,
    token: String,
    refresh_data: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct MonthViewParams {
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    pub modal: UseStateHandle<String>,
    pub context_menu_deps: ContextMenuDeps,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub token: String,
    pub refresh_data: Callback<()>,
}

#[function_component]
fn Day(props: &DayParams) -> Html {
    let key = format!(
        "{}-{}-{}-{}",
        props.day.clone(),
        props.active.clone(),
        props.show_right_edge.clone(),
        props.show_bottom_edge.clone()
    );
    let menu_x = use_state(|| 0);
    let menu_y = use_state(|| 0);

    let show_menu = props.context_menu_deps.context_menu.clone();
    let key_clone = key.clone();
    let menu_x_clone = menu_x.clone();
    let menu_y_clone = menu_y.clone();

    let on_context_menu = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            menu_x_clone.set(e.client_x());
            menu_y_clone.set(e.client_y());
            show_menu.set(key_clone.to_string());
        })
    };

    let context_menu_deps = props.context_menu_deps.clone();
    let new_event = props.new_event.clone();

    let handle_context_menu = move |index: usize| {
        if index == 0 {
            context_menu_deps.modal.set("Create Event".to_string());
            context_menu_deps.event.set(new_event.clone());
        }
        context_menu_deps.context_menu.set("None".to_string());
    };

    let inactive_style: String =
        "hover:bg-gray-200 hover:dark:bg-gray-800 text-gray-500".to_string();
    let today_style: String =
        "text-white hover:bg-gray-700 hover:dark:bg-gray-300 bg-black dark:text-black dark:bg-white".to_string();

    let style = if props.today {
        today_style
    } else if !props.active {
        inactive_style
    } else {
        "hover:bg-gray-200 hover:dark:bg-gray-800".to_string()
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
                class="flex justify-between px-2 lg:pb-1 py-2">
                <a>{ event.name.clone() }</a>
                <a>{ format_time(event.start.hour as i32, event.start.minute as i32) }</a>
                {
                    if *modal_2 == day_key_2 {
                        html! {
                            <ModalContainer
                                title={event.name.clone()}
                                component={html!{
                                    <EditEvent event={event.clone()}
                                        day_key={day_key_2.clone()}
                                        token={props.token.clone()}
                                        modal={modal_2.clone()}
                                        refresh_data={props.refresh_data.clone()} />
                                }}
                                modal={modal_2.clone()} />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        });
    }

    let conxt_menu_deps = props.context_menu_deps.clone();

    html! {
        <div class={ format!(
            "flex flex-col border-2 border-transparent {} {}",
            if props.show_right_edge { "border-r-gray-800" } else { "" },
            if props.show_bottom_edge { "border-b-gray-800" } else { "" }
        ) }>
            <a oncontextmenu={on_context_menu}
               class={ format!("text-4xl lg:text-xl {} px-4 lg:px-2 py-2 lg:py-1 m-1 rounded-full", style) }>
               { day }
            </a>
            <div class="flex flex-col w-full overflow-auto">
                { for event_elements.into_iter() }
            </div>
            if *conxt_menu_deps.context_menu == key {
                <RightClickMenu
                    x={*menu_x}
                    y={*menu_y}
                    options={vec![html! {"New event"}]}
                    return_index={handle_context_menu} />
            }
        </div>
    }
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

    let now = Local::now();
    let hour = now.hour();

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

        let new_event = CreateNewEvent {
            calendar_id: "".to_string(),
            name: "New event".to_string(),
            start: Time {
                year: *props.year as u16,
                month: *props.month as u8 - 1,
                day: (previous_month.days_in_month - i - 1) as u8,
                hour: hour as u8,
                minute: 0,
            },
            end: Time {
                year: *props.year as u16,
                month: *props.month as u8 - 1,
                day: (previous_month.days_in_month - i - 1) as u8,
                hour: hour as u8 + 1,
                minute: 0,
            },
        };

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={previous_month.days_in_month - i}
            active={false}
            today={is_today}
            show_right_edge={true}
            show_bottom_edge={true}
            modal={props.modal.clone()}
            context_menu_deps={props.context_menu_deps.clone()}
            new_event={new_event}
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

        let new_event = CreateNewEvent {
            calendar_id: "".to_string(),
            name: "New event".to_string(),
            start: Time {
                year: *props.year as u16,
                month: *props.month as u8,
                day: i as u8,
                hour: hour as u8,
                minute: 0,
            },
            end: Time {
                year: *props.year as u16,
                month: *props.month as u8,
                day: i as u8,
                hour: hour as u8 + 1,
                minute: 0,
            },
        };

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={i + 1}
            active={true}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
            modal={props.modal.clone()}
            context_menu_deps={props.context_menu_deps.clone()}
            new_event={new_event}
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
                format!("{}-{}-{}-{}", calendar.uuid, *props.year + 1, 0, i)
            } else {
                format!(
                    "{}-{}-{}-{}",
                    calendar.uuid,
                    *props.year,
                    *props.month + 1,
                    i
                )
            };

            day_keys.push(day_key);
        }

        let new_event = CreateNewEvent {
            calendar_id: "".to_string(),
            name: "New event".to_string(),
            start: Time {
                year: *props.year as u16,
                month: *props.month as u8 + 1,
                day: i as u8,
                hour: hour as u8,
                minute: 0,
            },
            end: Time {
                year: *props.year as u16,
                month: *props.month as u8 + 1,
                day: i as u8,
                hour: hour as u8 + 1,
                minute: 0,
            },
        };

        day_components.push(html! {<Day
            day_keys={day_keys.clone()}
            display_manager={display_manager.clone()}
            day={i + 1}
            active={false}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
            modal={props.modal.clone()}
            context_menu_deps={props.context_menu_deps.clone()}
            new_event={new_event}
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
