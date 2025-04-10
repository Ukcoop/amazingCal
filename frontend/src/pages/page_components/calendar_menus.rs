use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties, UseStateHandle};

use crate::core::{
    modal_functions::{create_event::CreateNewEvent, delete_calendar::delete_calendar},
    page_functions::calendar::ActiveCalendar,
};

use crate::components::modal::{
    modal_container::ModalContainer,
    modals::{
        confirm_action::ConfirmAction, create_calendar::CreateCalendar, create_event::CreateEvent,
    },
};

#[derive(PartialEq, Properties)]
pub struct CalendarMenusParams {
    pub token: UseStateHandle<String>,
    pub modal: UseStateHandle<String>,
    pub selected_calendar: UseStateHandle<String>,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub event: UseStateHandle<CreateNewEvent>,
    pub refresh_data: Callback<()>,
}

#[function_component]
pub fn CalendarMenus(props: &CalendarMenusParams) -> Html {
    let token = props.token.clone();
    let modal = props.modal.clone();
    let selected_calendar = props.selected_calendar.clone();
    let refresh_data = props.refresh_data.clone();

    let handle_delete_calendar = move |_event: MouseEvent| {
        let selected_calendar = selected_calendar.clone();
        let token = token.clone();
        let refresh_data = refresh_data.clone();
        let modal = modal.clone();

        spawn_local(async move {
            let code = delete_calendar(selected_calendar.to_string(), token.to_string()).await;
            if code == 200 {
                refresh_data.emit(());
                modal.set("None".to_string());
            }
        })
    };

    return html! {
        <div>
            {if props.modal.as_str() == "Create Calendar" {
                html!{
                    <ModalContainer title="Create Calendar" component={html!{
                        <CreateCalendar token={props.token.to_string()} modal={props.modal.clone()} refresh_data={props.refresh_data.clone()}/>
                    }} modal={props.modal.clone()}/>
                }
            } else { html!{} }}
            {if props.modal.as_str() == "Create Event" {
                html!{
                    <ModalContainer title="Create Event" component={html!{
                        <CreateEvent token={props.token.to_string()}
                            active_calendars={props.active_calendars.clone()}
                            event={props.event.clone()}
                            modal={props.modal.clone()}
                            refresh_data={props.refresh_data.clone()}/>
                    }} modal={props.modal.clone()}/>
                }
            } else { html!{} }}
            {if props.modal.as_str() == "Delete Calendar" {
                html!{
                    <ModalContainer title="Delete Calendar" component={html!{
                        <ConfirmAction text="Are you sure you want to delete this calendar?" action={handle_delete_calendar}/>
                    }} modal={props.modal.clone()}/>
                }
            } else { html!{} }}
        </div>
    };
}
