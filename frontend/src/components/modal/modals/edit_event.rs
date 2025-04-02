use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Html, Properties, UseStateHandle};

use crate::core::{
    page_functions::event::{edit_event, use_get_states},
    shared::Event,
    time::{format_time, get_month_name, get_ordinal},
};

use crate::components::{
    main::{
        button::{Button, ButtonStyle},
        input_field::InputField,
        status::{Status, StatusCode, StatusObject},
    },
    modal::time_editor::TimeEditor,
};

#[derive(Properties, PartialEq)]
pub struct EditEventParams {
    pub event: Event,
    pub day_key: String,
    pub token: String,
    pub modal: UseStateHandle<String>,
}

#[function_component]
pub fn EditEvent(props: &EditEventParams) -> Html {
    let event = props.event.clone();

    let editing = use_state(|| false);
    let open = use_state(|| "None".to_string());
    let status = use_state(|| StatusObject {
        code: StatusCode::Ok,
        data: "".to_string(),
    });

    let name = use_state(|| event.name.clone());

    let start_states = use_get_states(event.start.clone());
    let end_states = use_get_states(event.end.clone());

    let start_day = format!(
        "{} of {}",
        get_ordinal((event.start.day + 1) as i32),
        get_month_name(event.start.month as i32)
    );

    let end_day = format!(
        "{} of {}",
        get_ordinal((event.end.day + 1) as i32),
        get_month_name(event.end.month as i32)
    );

    let start_time = format_time(event.start.hour as i32, event.start.minute as i32);
    let end_time = format_time(event.end.hour as i32, event.end.minute as i32);

    let token = props.token.clone();
    let status_clone = status.clone();

    let name_clone = name.clone();
    let uuid_clone = event.uuid.clone();
    let modal = props.modal.clone();

    let start_states_clone = start_states.clone();
    let end_states_clone = end_states.clone();

    let handle_submit = move |_| {
        let name_clone = name_clone.clone();
        let uuid_clone = uuid_clone.clone();

        let start_states_clone = start_states_clone.clone();
        let end_states_clone = end_states_clone.clone();

        let token_clone = token.clone();
        let status_clone = status_clone.clone();
        let modal = modal.clone();

        status_clone.set(StatusObject {
            code: StatusCode::Loading,
            data: "Editing event...".to_string(),
        });

        spawn_local(async move {
            let code = edit_event(
                name_clone.to_string(),
                uuid_clone.to_string(),
                start_states_clone.clone(),
                end_states_clone.clone(),
                token_clone.to_string(),
            )
            .await;

            if code == 200 {
                status_clone.set(StatusObject {
                    code: StatusCode::Ok,
                    data: "Event edited successfully".to_string(),
                });

                modal.set("None".to_string());
            } else {
                status_clone.set(StatusObject {
                    code: StatusCode::Error,
                    data: format!("Error editing event: {}", code),
                });
            }
        });
    };

    html! {
        <div class="w-96">
            <div class="flex justify-between mt-4">
                <a>{ "Starts:" }</a>
                <div class="flex">{ format!("{}, {}", start_day, start_time) }</div>
            </div>
            <div class="flex justify-between">
                <a>{ "Ends:" }</a>
                <div class="flex">{ format!("{}, {}", end_day, end_time) }</div>
            </div>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            {
                if *editing {
                    html! {
                        <div>
                            <div class="flex justify-between items-center">
                                <a>{ "Name:" }</a>
                                <div class="flex justify-end w-48">
                                    <InputField<String> varient="text" value={ name.clone() } />
                                </div>
                            </div>
                            <TimeEditor id="Start" event={ event.clone() } open={ open.clone() } states={ start_states.clone() } />
                            <TimeEditor id="End" event={ event.clone() } open={ open.clone() } states={ end_states.clone() } />
                            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
                            <Button style={ ButtonStyle::Primary } width="" on_click={ handle_submit }>{ "Submit" }</Button>
                            <Status status={status.clone()} />
                        </div>
                    }
                } else {
                    html! {
                        <Button style={ ButtonStyle::Primary } width="" on_click={ move |_| editing.set(true) }>{ "Edit" }</Button>
                    }
                }
            }
        </div>
    }
}
