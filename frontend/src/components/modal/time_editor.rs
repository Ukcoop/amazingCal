use yew::{function_component, html, use_effect_with, use_state, Html, Properties, UseStateHandle};

use crate::{
    components::main::{dropdown::DropDown, input_field::InputField},
    core::{
        calendar_data::get_year_data,
        shared::Event,
        time::{get_month_name, get_ordinal},
    },
};

#[derive(PartialEq, Clone)]
pub struct States {
    pub day: UseStateHandle<u8>,
    pub month: UseStateHandle<u8>,
    pub year: UseStateHandle<u16>,
    pub hour: UseStateHandle<u8>,
    pub minute: UseStateHandle<u8>,
    pub ampm: UseStateHandle<u8>,
}

#[derive(PartialEq, Clone)]
pub struct StatesContainer {
    pub start: States,
    pub end: States,
}

#[derive(Properties, PartialEq)]
pub struct TimeEditorParams {
    pub id: String,
    pub event: Event,
    pub open: UseStateHandle<String>,
    pub states: States,
}

#[function_component]
pub fn TimeEditor(props: &TimeEditorParams) -> Html {
    let event = props.event.clone();

    let day_options = use_state(Vec::new);
    let mut month_options = vec![];
    let mut year_options = vec![];
    let mut hour_options = vec![];
    let mut minute_options = vec![];

    let year_data = get_year_data(*props.states.year as i32);

    let states = props.states.clone();
    let day_options_clone = day_options.clone();
    use_effect_with((states.month.clone(), states.year.clone()), move |_| {
        let mut new_day_options = vec![];

        for i in 0..year_data[*states.month.clone() as usize].days_in_month {
            new_day_options
                .push(html! { <a class="text-4xl lg:text-base">{ get_ordinal(i + 1) }</a> });
        }

        day_options_clone.set(new_day_options);

        if *states.day >= year_data[*states.month.clone() as usize].days_in_month as u8 {
            states
                .day
                .set(year_data[*states.month.clone() as usize].days_in_month as u8 - 1);
        }
    });

    for i in 0..12 {
        month_options.push(html! { <a class="text-4xl lg:text-base">{ get_month_name(i) }</a> });
    }

    for i in 0..8 {
        year_options.push(
            html! { <a class="text-4xl lg:text-base">{ *props.states.year.clone() + i }</a> },
        );
    }

    hour_options.push(html! { <a class="text-4xl lg:text-base">{"12"}</a> });
    for i in 1..12 {
        hour_options.push(html! { <a class="text-4xl lg:text-base">{ i }</a> });
    }

    for i in 0..60 {
        minute_options.push(html! { <a class="text-4xl lg:text-base">{ format!("{:02}", i) }</a> });
    }

    let day = props.states.day.clone();
    let month = props.states.month.clone();
    let year = props.states.year.clone();
    let hour = props.states.hour.clone();
    let minute = props.states.minute.clone();
    let ampm = props.states.ampm.clone();

    let handle_edit = move |id: String, index: usize| match id.as_str() {
        "edit-day" => day.set(index as u8),
        "edit-month" => month.set(index as u8),
        "edit-year" => year.set(event.start.year + index as u16),
        "edit-hour" => hour.set(index as u8),
        "edit-minute" => minute.set(index as u8),
        "edit-ampm" => ampm.set(index as u8),
        _ => {}
    };

    let handle_edit_clone_a = handle_edit.clone();
    let handle_edit_clone_b = handle_edit.clone();
    let handle_edit_clone_c = handle_edit.clone();
    let handle_edit_clone_d = handle_edit.clone();

    return html! {
        <div class="flex justify-between mt-2">
            <a class="my-1 text-4xl lg:text-base">{ format!("{}:", props.id) }</a>
            <div class="flex justify-end items-center w-max">
                <DropDown
                    open={ props.open.clone() }
                    id={format!("edit-{}-day", props.id)}
                    minimal={ true }
                    element={ html!{ <a class="text-4xl lg:text-base">{ get_ordinal(*props.states.day.clone() as i32 + 1) }</a> } }
                    options={ (*day_options).clone() }
                    return_index={ move |index: usize| { handle_edit_clone_a("edit-day".to_string(), index) } }
                />
                <a class="my-1 text-4xl lg:text-base">{ "of" }</a>
                <DropDown
                    open={ props.open.clone() }
                    id={format!("edit-{}-month", props.id)}
                    minimal={ true }
                    element={ html!{ <a class="text-4xl lg:text-base">{ get_month_name(*props.states.month.clone() as i32) }</a> } }
                    options={ month_options }
                    return_index={ move |index: usize| { handle_edit_clone_b("edit-month".to_string(), index) } }
                />
                <a class="my-1 text-4xl lg:text-base">{ "," }</a>
                <div class="w-28 lg:w-14 px-1">
                    <InputField<u16> varient="number" value={ props.states.year.clone() } />
                </div>
                <a class="my-1 text-4xl lg:text-base">{ "@" }</a>
                <DropDown
                    open={ props.open.clone() }
                    id={format!("edit-{}-hour", props.id)}
                    minimal={ true }
                    element={ html!{ <a class="text-4xl lg:text-base">{ if *props.states.hour.clone() == 0 { 12 } else { *props.states.hour.clone() } }</a> } }
                    options={ hour_options }
                    return_index={ move |index: usize| { handle_edit_clone_c("edit-hour".to_string(), index) } }
                />
                <a class="my-1 text-4xl lg:text-base">{ ":" }</a>
                <DropDown
                    open={ props.open.clone() }
                    id={format!("edit-{}-minute", props.id)}
                    minimal={ true }
                    element={ html!{ <a class="text-4xl lg:text-base">{ format!("{:02}", *props.states.minute.clone()) }</a> } }
                    options={ minute_options }
                    return_index={ move |index: usize| { handle_edit_clone_d("edit-minute".to_string(), index) } }
                />
                <DropDown
                    open={ props.open.clone() }
                    id={format!("edit-{}-ampm", props.id)}
                    minimal={ true }
                    element={ html!{ <a class="text-4xl lg:text-base">{ if *props.states.ampm.clone() == 0 { "am" } else { "pm" } }</a> } }
                    options={ vec![
                        html!{ <a class="text-4xl lg:text-base">{ "am" }</a> },
                        html!{ <a class="text-4xl lg:text-base">{ "pm" }</a> }
                    ] }
                    return_index={ move |index: usize| { handle_edit("edit-ampm".to_string(), index) } }
                />
            </div>
        </div>
    };
}
