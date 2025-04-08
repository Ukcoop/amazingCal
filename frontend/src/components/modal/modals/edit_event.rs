use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::core::{
    modal_functions::edit_event::{handle_delete, handle_submit, use_get_states},
    shared::Event,
    time::{format_time, get_month_name, get_ordinal},
};

use crate::components::{
    main::{
        button::{Button, ButtonStyle},
        input_field::InputField,
        status::{Status, StatusCode, StatusObject},
    },
    modal::{
        modal_container::ModalContainer,
        modals::confirm_action::ConfirmAction,
        time_editor::{StatesContainer, TimeEditor},
    },
};

#[derive(Properties, PartialEq)]
pub struct EditEventParams {
    pub event: Event,
    pub day_key: String,
    pub token: String,
    pub modal: UseStateHandle<String>,
    pub refresh_data: Callback<()>,
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

    let open_clone = open.clone();
    let handle_confirm = move |_| {
        open_clone.set("Confirm Action".to_string());
    };

    let day_key = props.day_key.clone();
    let day_key_clone = day_key.clone();

    let name_clone = name.clone();
    let token_clone = token.clone();

    let uuid_clone_a = event.uuid.clone();
    let uuid_clone_b = event.uuid.clone();

    let status_clone = status.clone();
    let open_clone = open.clone();

    let modal_clone_a = props.modal.clone();
    let modal_clone_b = props.modal.clone();

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
                            <Button style={ ButtonStyle::Primary } width="" on_click={ move |_| {
                                handle_submit(
                                    name_clone.to_string(),
                                    uuid_clone_a.to_string(),
                                    StatesContainer {
                                        start: start_states.clone(),
                                        end: end_states.clone(),
                                    },
                                    day_key.to_string(),
                                    token_clone.to_string(),
                                    status_clone.clone(),
                                    modal_clone_a.clone(),
                                );
                            } }>{ "Submit" }</Button>
                            <Button style={ ButtonStyle::Secondary } width="" on_click={ handle_confirm }>{ "Delete" }</Button>
                            <Status status={status.clone()} />
                            {if *open == "Confirm Action" {
                                html! {
                                    <ModalContainer
                                        title="Confirm Action"
                                        component={
                                            html! {
                                                <ConfirmAction
                                                    text="Are you sure you want to delete this event?"
                                                    action={ move |_| {
                                                        handle_delete(
                                                            uuid_clone_b.to_string(),
                                                            token.to_string(),
                                                            day_key_clone.to_string(),
                                                            status.clone(),
                                                            modal_clone_b.clone(),
                                                            open_clone.clone(),
                                                        );
                                                        }
                                                    }
                                                />
                                            }
                                        }
                                    modal={
                                        open.clone()
                                    }
                                />}} else {html!{""}}}
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
