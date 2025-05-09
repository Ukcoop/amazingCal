use std::fmt::Display;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use yew::{
    function_component, html, Callback, Html, InputEvent, KeyboardEvent, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct InputFieldParams<T: PartialEq + Display + FromStr> {
    pub varient: String,
    pub value: UseStateHandle<T>,
    #[prop_or_default]
    pub min: String,
    #[prop_or_else(|| Callback::from(|_| ())) ]
    pub on_key_press: Callback<KeyboardEvent>,
}

#[function_component]
pub fn InputField<T: PartialEq + Display + FromStr + 'static>(props: &InputFieldParams<T>) -> Html {
    let on_change = {
        let value = props.value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                if let Ok(input_element) = target.dyn_into::<web_sys::HtmlInputElement>() {
                    let value_str = input_element.value();
                    if let Ok(parsed_value) = value_str.parse::<T>() {
                        value.set(parsed_value);
                    }
                }
            }
        })
    };

    html! {
        <div class="w-full">
            <input
                class="w-full border border-black dark:border-gray-600 rounded-md p-1 h-18 text-4xl lg:text-base lg:h-9"
                type={props.varient.clone()}
                value={props.value.to_string()}
                oninput={on_change}
                min={props.min.clone()}
                onkeypress={props.on_key_press.clone()}
            />
        </div>
    }
}
