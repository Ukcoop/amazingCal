use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

use crate::components::main::button::{Button, ButtonStyle};

#[derive(Properties, PartialEq)]
pub struct ConfirmActionParams {
    pub text: String,
    pub action: Callback<MouseEvent>,
}

#[function_component]
pub fn ConfirmAction(props: &ConfirmActionParams) -> Html {
    return html! {
        <div>
            <a>{&props.text}</a>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <Button style={ButtonStyle::Primary} width="" on_click={props.action.clone()}>{"Confirm"}</Button>
        </div>
    };
}
