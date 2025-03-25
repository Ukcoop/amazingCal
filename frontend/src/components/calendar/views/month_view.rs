use yew::{function_component, html, virtual_dom::VNode, Html, Properties, UseStateHandle};

use crate::core::{
    calendar_data::{get_todays_date, get_year_data},
    event_manager::EventDisplayManager,
    shared::Event,
};

#[derive(Properties, PartialEq)]
struct DayParams {
    day_key: String,
    events: Vec<Event>,
    day: i32,
    active: bool,
    today: bool,
    show_right_edge: bool,
    show_bottom_edge: bool,
    // needed when modals are implemented: modal: String,
}

#[derive(Properties, PartialEq)]
pub struct MonthViewParams {
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    // needed when modals are implemented: pub modal: UseStateHandle<String>,
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

    return html! {
        <div class={
            format!("flex flex-col border-2 border-transparent {} {}",
                if props.show_right_edge {"border-r-gray-800"} else {""},
                if props.show_bottom_edge {"border-b-gray-800"} else {""})
        }>
            <a class={format!("text-xl {} px-2 py-1 m-1", style)}>{props.day}</a>
            <div class="flex flex-col w-full overflow-auto">
            {
                props.events.iter().map(|event| html! {
                    <div class="flex justify-between px-2 pb-1">
                        <a>{event.name.clone()}</a>
                        <a>{format!("{}:{}", event.start.hour, format!("{:0>2}", event.start.minute))}</a>
                        // modal stuff goes here
                    </div>
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
            return html! { <div>{"Error loading calendar, could not get display manager."}</div> };
        }
    };

    let current_month = get_year_data(*props.year)[*props.month as usize].clone();
    let previous_month = if *props.month == 0 {
        get_year_data(*props.year - 1)[11].clone()
    } else {
        get_year_data(*props.year)[*props.month as usize - 1].clone()
    };

    let (todays_month, todays_year, todays_day) = get_todays_date();

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

    for i in (0..current_month.week_index).rev() {
        let is_today = todays_month == *props.month - (1_i32)
            && todays_year == *props.year
            && previous_month.days_in_month - i + 1 == todays_day;

        let day_key = if *props.month == 0 {
            format!(
                "default-{}-{}-{}",
                *props.year - 1,
                11,
                previous_month.days_in_month - i + 1
            )
        } else {
            format!(
                "default-{}-{}-{}",
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
        />});
    }

    for i in 0..current_month.days_in_month {
        let is_today =
            todays_month == *props.month && todays_year == *props.year && i + 1 == todays_day;

        let day_key = format!("default-{}-{}-{}", *props.year, *props.month, i);

        let show_right_edge = (current_month.week_index + i + 1) % 7 != 0;
        let show_bottom_edge = current_month.week_index + i < days - 7;

        day_components.push(html! {<Day
            day_key={day_key.clone()}
            events={display_manager.get_events_by_key(&day_key).clone()}
            day={i + 1}
            active={true}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
        />});
    }

    for i in 0..(days - current_month.week_index - current_month.days_in_month) {
        let is_today = todays_month == *props.month + (1_i32)
            && todays_year == *props.year
            && i + 1 == todays_day;

        let day_key = format!("default-{}-{}-{}", *props.year, *props.month + 1, i);

        let show_right_edge =
            (current_month.week_index + current_month.days_in_month + i + 1) % 7 != 0;
        let show_bottom_edge =
            current_month.week_index + current_month.days_in_month + i < days - 7;

        day_components.push(html! {<Day
            day_key={day_key.clone()}
            events={display_manager.get_events_by_key(&day_key).clone()}
            day={i + 1}
            active={false}
            today={is_today}
            show_right_edge={show_right_edge}
            show_bottom_edge={show_bottom_edge}
        />});
    }

    return html! {
        <div class={format!("grid grid-cols-7 {} content-start w-full h-full border-2 border-black dark:border-transparent rounded-lg dark:bg-gray-900", rows)}>
            {day_components}
        </div>
    };
}
