use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::{
    components::{
        main::{
            button::{Button, ButtonStyle},
            dropdown::DropDown,
            input_field::InputField,
            status::{Status, StatusCode, StatusObject},
        },
        modal::time_editor::{StatesContainer, TimeEditor},
    },
    core::{modal_functions::create_event::CreateNewEvent, shared::Event},
};

use crate::core::{
    modal_functions::{
        create_event::{get_calendar_options, handle_calendar_change, handle_submit},
        edit_event::use_get_states,
    },
    page_functions::calendar::ActiveCalendar,
};

#[derive(Properties, PartialEq)]
pub struct CreateEventParams {
    pub token: String,
    pub active_calendars: UseStateHandle<Vec<ActiveCalendar>>,
    pub event: UseStateHandle<CreateNewEvent>,
    pub modal: UseStateHandle<String>,
    pub refresh_data: Callback<()>,
}

#[function_component]
pub fn CreateEvent(props: &CreateEventParams) -> Html {
    let open = use_state(|| "None".to_string());
    let open_dropdown = use_state(|| "None".to_string());

    let active_calendar = props.active_calendars.clone();
    let active_calendar_index: UseStateHandle<usize> = use_state(|| 0);
    let active_calendar_options: Vec<Html> = get_calendar_options(props.active_calendars.clone());

    let status = use_state(|| StatusObject {
        code: StatusCode::Ok,
        data: "".to_string(),
    });

    let name = use_state(|| props.event.name.clone());

    let start_states = use_get_states(props.event.start.clone());
    let end_states = use_get_states(props.event.end.clone());

    let token = props.token.clone();

    let modal = props.modal.clone();
    let calendar_id = props.active_calendars[0].uuid.clone();
    let status_clone = status.clone();

    let event = Event {
        uuid: "".to_string(),
        name: props.event.name.clone(),
        start: props.event.start.clone(),
        end: props.event.end.clone(),
    };

    let refresh_data = props.refresh_data.clone();

    html! {
        <div class="w-96 pt-1">
            <div class="flex justify-between items-center py-1">
                <a>{ "Calendar:" }</a>
                <DropDown
                    open={ open_dropdown.clone() }
                    id="Selected_Calendar"
                    minimal={ false }
                    element={ html! { <a>{active_calendar[*active_calendar_index.clone()].name.clone()}</a> }}
                    options={active_calendar_options.clone()}
                    return_index={move |index: usize| {
                        handle_calendar_change(index, active_calendar_index.clone(), open_dropdown.clone())
                    }}
                />
            </div>
            <div class="flex justify-between items-center">
                <a>{ "Name:" }</a>
                <div class="flex justify-end w-48">
                    <InputField<String> varient="text" value={ name.clone() } />
                </div>
            </div>
            <TimeEditor
                id="Start"
                event={ event.clone() }
                open={ open.clone() }
                states={ start_states.clone() }
            />
            <TimeEditor
                id="End"
                event={ event.clone() }
                open={ open.clone() }
                states={ end_states.clone() }
            />
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <Button style={ ButtonStyle::Primary } width="" on_click={ move |_| {
                handle_submit(
                    name.to_string(),
                    calendar_id.clone(),
                    modal.clone(),
                    refresh_data.clone(),
                    StatesContainer {
                        start: start_states.clone(),
                        end: end_states.clone(),
                    },
                    token.clone(),
                    status_clone.clone(),
        )
    } }>{ "Submit" }</Button>
            <Status status={status.clone()} />
        </div>
    }
}
