use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

use crate::components::main::{
    button::{Button, ButtonStyle},
    input_field::InputField,
    status::{StatusCode, StatusObject},
};

use crate::core::modal_functions::create_calendar::handle_submit;

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

    let token = props.token.clone();
    let modal = props.modal.clone();
    let refresh_data = props.refresh_data.clone();

    html! {
        <div class="pt-1">
            <div class="flex justify-between items-center">
                <a>{ "Name:" }</a>
                <div class="flex justify-end w-48">
                    <InputField<String> varient="text" value={ name.clone() } />
                </div>
            </div>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <Button style={ ButtonStyle::Primary } width="" on_click={ move |_| {
                handle_submit(name.to_string(), token.clone(), modal.clone(), status.clone(), refresh_data.clone())
            }}>{ "Submit" }</Button>
        </div>
    }
}
