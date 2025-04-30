use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, MouseEvent, Properties,
    UseStateHandle,
};

use crate::core::time::get_month_name;

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    pub async fn get_email() -> JsValue;
    pub fn handle_signout();
}

use crate::components::main::{
    button::{Button, ButtonStyle},
    dropdown::DropDown,
    material_symbols::MaterialSymbol,
};

#[derive(PartialEq, Properties)]
pub struct TopCalendarBarParams {
    pub menu: UseStateHandle<bool>,
    pub month: UseStateHandle<i32>,
    pub year: UseStateHandle<i32>,
    pub modal: UseStateHandle<String>,
}

#[function_component]
pub fn TopCalendarBar(props: &TopCalendarBarParams) -> Html {
    const CLICKABLE_ELEMENT_CLASS: &str =
        "flex items-center justify-center p-4 lg:p-1 rounded-md hover:bg-gray-200 hover:dark:bg-gray-900";

    let open = use_state(|| "None".to_string());
    let email = use_state(|| "".to_string());

    let email_clone = email.clone();
    use_effect_with((), move |_| {
        spawn_local(async move {
            let new_email = get_email().await;
            email_clone.set(match new_email.as_string() {
                Some(email) => email,
                _ => "".to_string(),
            });
        });
    });

    let modal = props.modal.clone();
    let add_event = {
        Callback::from(move |_event: MouseEvent| {
            modal.set("Create Event".to_string());
        })
    };

    let menu_clone = props.menu.clone();
    let toggle_menu = {
        Callback::from(move |_event: MouseEvent| {
            menu_clone.set(!*menu_clone);
        })
    };

    let month_clone = props.month.clone();
    let year_clone = props.year.clone();
    let backward_one_month = {
        Callback::from(move |_event: MouseEvent| {
            if *month_clone == 0 {
                month_clone.set(11);
                year_clone.set(*year_clone - 1);
            } else {
                month_clone.set(*month_clone - 1);
            }
        })
    };

    let month_clone = props.month.clone();
    let year_clone = props.year.clone();
    let foward_one_month = {
        Callback::from(move |_event: MouseEvent| {
            if *month_clone == 11 {
                month_clone.set(0);
                year_clone.set(*year_clone + 1);
            } else {
                month_clone.set(*month_clone + 1);
            }
        })
    };

    let open_clone = open.clone();
    let handle_account_menu = move |index: usize| {
        if index == 0 {
            handle_signout();
        }
        open_clone.set("None".to_string());
    };

    return html! {
        <div class="w-full h-20 lg:h-10 mb-2 flex items-center justify-between">
            <div class="flex items-center">
                <Button style={ButtonStyle::Secondary} width="w-max" on_click={toggle_menu} responsive_hide={"hidden lg:block".to_string()}>
                    <MaterialSymbol name="menu"/>
                </Button>
                <Button style={ButtonStyle::Secondary} width="w-max" on_click={add_event} responsive_hide={"block lg:hidden".to_string()}>
                    <MaterialSymbol name="add"/>
                    {if *props.menu { html!{"Event"} } else { html!{""} }}
                </Button>
                <a class="text-4xl lg:text-2xl pl-4 pr-2" href="/">{"amazingCal"}</a>
                <div class="flex px-2">
                    <div class={CLICKABLE_ELEMENT_CLASS} onclick={backward_one_month}>
                        <MaterialSymbol name="arrow_back_ios_new"/>
                    </div>
                    <div class={CLICKABLE_ELEMENT_CLASS} onclick={foward_one_month}>
                        <MaterialSymbol name="arrow_forward_ios"/>
                    </div>
                </div>
                <a class="text-4xl lg:text-2xl">{format!("{} {:?}", get_month_name(*props.month), *props.year)}</a>
            </div>
            <div class="flex items-center">
              <div class="hidden lg:block">
                <DropDown open={open.clone()} id="Account" minimal={false} element={html!{
                    <svg width="40" height="40" data-jdenticon-value={(*email).clone()}></svg>
                }} options={vec![html!{"Sign out"}]} return_index={handle_account_menu}/>
              </div>
                <Button style={ButtonStyle::Secondary} width="w-max" on_click={|_| {handle_signout();}} responsive_hide={"block lg:hidden".to_string()}>
                  {"Sign out"}
                </Button>
            </div>
        </div>
    };
}
