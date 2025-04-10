use yew::{
    function_component, html, use_state, Callback, Html, MouseEvent, Properties, UseStateHandle,
};

use crate::core::page_functions::calendar::ActiveCalendar;
use crate::core::shared::Calendar;

use crate::components::main::{
    button::{Button, ButtonStyle},
    check_box::CheckBox,
    material_symbols::MaterialSymbol,
    right_click_menu::RightClickMenu,
};

#[derive(PartialEq, Properties)]
pub struct SideCalendarBarParams {
    pub menu: UseStateHandle<bool>,
    pub context_menu: UseStateHandle<String>,
    pub modal: UseStateHandle<String>,
    pub calendars: UseStateHandle<Vec<Calendar>>,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub selected_calendar: UseStateHandle<String>,
}

#[function_component]
pub fn SideCalendarBar(props: &SideCalendarBarParams) -> Html {
    const CLICKABLE_ELEMENT_CLASS: &str =
        "flex items-center justify-center p-1 rounded-md hover:bg-gray-200 hover:dark:bg-gray-900";

    let menu_x = use_state(|| 0);
    let menu_y = use_state(|| 0);

    let modal = props.modal.clone();
    let add_event = {
        Callback::from(move |_event: MouseEvent| {
            modal.set("Create Event".to_string());
        })
    };

    let modal = props.modal.clone();
    let add_calendar = {
        Callback::from(move |_event: MouseEvent| {
            modal.set("Create Calendar".to_string());
        })
    };

    let modal = props.modal.clone();
    let context_menu = props.context_menu.clone();
    let handle_context_menu = move |index: usize| {
        if index == 0 {
            modal.set("Delete Calendar".to_string());
        }
        context_menu.set("None".to_string());
    };

    let active_calendars = props.active_calendars.clone();
    let toggle_active_calendars = move |(name, uuid): (String, String)| {
        let mut new_active_calendars = (*active_calendars).clone();
        if let Some(index) = new_active_calendars.iter().position(|cal| cal.uuid == uuid) {
            new_active_calendars.remove(index);
        } else {
            new_active_calendars.push(ActiveCalendar { name, uuid });
        }
        active_calendars.set(new_active_calendars);
    };

    return html! {
        <div class={format!("flex flex-col {} h-full mr-2", if *props.menu {"w-60"} else {"w-15"})}>
            <Button style={ButtonStyle::Secondary} width="w-max" on_click={add_event}>
                <MaterialSymbol name="add"/>
                {if *props.menu { html!{"Event"} } else { html!{""} }}
            </Button>
            {if *props.menu {
                html! {
                    <div>
                        <div class="flex justify-between items-center my-2">
                            <p>{"Calendars"}</p>
                            <div class={format!("flex items-center {}", CLICKABLE_ELEMENT_CLASS)}>
                                <a onclick={add_calendar}>
                                    <MaterialSymbol name="add"/>
                                </a>
                            </div>
                        </div>
                        <div class="flex flex-col">
                            {
                                props.calendars.iter().map(|calendar| {
                                    let menu_x = menu_x.clone();
                                    let menu_y = menu_y.clone();
                                    let context_menu = props.context_menu.clone();
                                    let selected_calendar = props.selected_calendar.clone();
                                    let name = calendar.name.clone();
                                    let uuid = calendar.uuid.clone();
                                    let toggle_active_calendars = toggle_active_calendars.clone();

                                    html! {
                                        <div class="flex items-center">
                                            <CheckBox start_state={true} name={name.clone()} uuid={uuid.clone()} toggle={toggle_active_calendars}/>
                                            <a oncontextmenu={move |e: MouseEvent| {
                                                e.prevent_default();
                                                menu_x.set(e.client_x());
                                                menu_y.set(e.client_y());
                                                context_menu.set("Calendar".to_string());
                                                selected_calendar.set(uuid.clone());
                                            }} class="pl-1 text-lg">
                                                { &*calendar.name }
                                            </a>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                }
            } else { html!{} }}
            if *props.context_menu == "Calendar" {
                <RightClickMenu
                    x={*menu_x}
                    y={*menu_y}
                    options={vec![html! {"Delete calendar"}]}
                    return_index={handle_context_menu}
                />
            }
        </div>
    };
}
