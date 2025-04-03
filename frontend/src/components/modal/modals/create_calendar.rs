use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::{
    components::main::{
        button::{Button, ButtonStyle},
        input_field::InputField,
        status::{StatusCode, StatusObject},
    },
    core::page_functions::calendar::create_calendar,
};

#[derive(Properties, PartialEq)]
pub struct CreateCalendarParams {
    pub token: String,
    pub modal: UseStateHandle<String>,
    pub refresh_data: Callback<()>,
}

#[function_component]
pub fn CreateCalendar(props: &CreateCalendarParams) -> Html {
    let status = use_state(|| StatusObject {
        code: StatusCode::Ok,
        data: "".to_string(),
    });

    let name = use_state(|| "Calendar".to_string());

    let name_clone = name.clone();
    let token_clone = props.token.clone();
    let modal = props.modal.clone();
    let refresh_data = props.refresh_data.clone();

    let handle_submit = move |_| {
        let name_clone = name_clone.clone();
        let status_clone = status.clone();
        let token_clone = token_clone.clone();
        let modal = modal.clone();
        let refresh_data = refresh_data.clone();

        spawn_local(async move {
            let code = create_calendar(name_clone.to_string(), token_clone.to_string()).await;

            if code == 200 {
                status_clone.set(StatusObject {
                    code: StatusCode::Ok,
                    data: "Event edited successfully".to_string(),
                });

                modal.set("None".to_string());
                refresh_data.emit(());
            } else {
                status_clone.set(StatusObject {
                    code: StatusCode::Error,
                    data: format!("Error editing event: {}", code),
                });
            }
        });
    };

    html! {
        <div class="pt-1">
            <div class="flex justify-between items-center">
                <a>{ "Name:" }</a>
                <div class="flex justify-end w-48">
                    <InputField<String> varient="text" value={ name.clone() } />
                </div>
            </div>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <Button style={ ButtonStyle::Primary } width="" on_click={ handle_submit }>{ "Submit" }</Button>
        </div>
    }
}
