use once_cell::sync::Lazy;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, MouseEvent,
    UseStateHandle,
};
use yew_router::hooks::use_navigator;

use chrono::NaiveDate;

use crate::core::shared::Calendar;

use crate::core::page_functions::calendar::{get_current_session, get_user_data};

use crate::components::main::{
    button::{Button, ButtonStyle},
    dropdown::DropDown,
    material_symbols::MaterialSymbol,
};

use crate::components::calendar::view_switcher::CalendarView;

static PUBLIC_SUPABASE_URL: Lazy<&str> =
    Lazy::new(|| option_env!("PUBLIC_SUPABASE_URL").unwrap_or(""));
static PUBLIC_ANON_KEY: Lazy<&str> = Lazy::new(|| option_env!("PUBLIC_ANON_KEY").unwrap_or(""));

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    pub fn init_supabase(supabase_url: String, anon_key: String);
    pub async fn get_session() -> JsValue;
    pub async fn get_email() -> JsValue;
    pub fn handle_signout();
}

pub fn get_month_name(month_number: i32) -> String {
    let month = month_number + 1;
    match NaiveDate::from_ymd_opt(2023, month as u32, 1) {
        Some(date) => date.format("%B").to_string(),
        None => "Invalid month".to_string(),
    }
}

#[function_component]
pub fn CalendarPage() -> Html {
    let navigator = use_navigator();
    init_supabase(PUBLIC_SUPABASE_URL.to_string(), PUBLIC_ANON_KEY.to_string());

    let token = use_state(|| "".to_string());
    let calendars: UseStateHandle<Vec<Calendar>> = use_state(Vec::new);

    let menu = use_state(|| false);
    let open = use_state(|| "None".to_string());
    let view = use_state(|| "Month".to_string());
    let email = use_state(|| "".to_string());

    let month: UseStateHandle<i32> = use_state(|| 2);
    let year: UseStateHandle<i32> = use_state(|| 2025);

    let navigator_clone = navigator.clone();
    let calendars_clone = calendars.clone();

    let token_clone_a = token.clone();
    let token_clone_b = token.clone();

    let month_clone_a = month.clone();
    let month_clone_b = month.clone();

    let year_clone_a = year.clone();
    let year_clone_b = year.clone();

    let menu_clone = menu.clone();
    let email_clone = email.clone();

    use_effect_with((), move |_| get_current_session(navigator, token_clone_a));
    use_effect_with(token, move |_| {
        get_user_data(calendars, navigator_clone, token_clone_b)
    });

    use_effect_with((), move |_| {
        spawn_local(async move {
            let new_email = get_email().await;
            email_clone.set(match new_email.as_string() {
                Some(email) => email,
                None => "".to_string(),
            });
        });
    });

    let empty_callback = { Callback::from(|_event: MouseEvent| {}) };
    const CLICKABLE_ELEMENT_CLASS: &str =
        "flex items-center justify-center p-1 rounded-md hover:bg-gray-200 hover:dark:bg-gray-900";

    let toggle_menu = {
        Callback::from(move |_event: MouseEvent| {
            menu_clone.set(!*menu_clone);
        })
    };

    let backward_one_month = {
        Callback::from(move |_event: MouseEvent| {
            if *month_clone_a == 0 {
                month_clone_a.set(11);
                year_clone_a.set(*year_clone_a - 1);
            } else {
                month_clone_a.set(*month_clone_a - 1);
            }
        })
    };

    let foward_one_month = {
        Callback::from(move |_event: MouseEvent| {
            if *month_clone_b == 11 {
                month_clone_b.set(0);
                year_clone_b.set(*year_clone_b + 1);
            } else {
                month_clone_b.set(*month_clone_b + 1);
            }
        })
    };

    let view_clone = view.clone();
    let open_clone = open.clone();
    let view_options = vec![html! {"Month"}, html! {"Schedule"}];

    let handle_view_menu = move |index: usize| {
        match index {
            0 => view_clone.set("Month".to_string()),
            1 => view_clone.set("Schedule".to_string()),
            _ => view_clone.set("Week".to_string()),
        }

        open_clone.set("None".to_string());
    };

    let open_clone = open.clone();
    let handle_account_menu = move |index: usize| {
        if index == 0 {
            handle_signout();
        }

        open_clone.set("None".to_string());
    };

    let add_calendar = { Callback::from(move |_event: MouseEvent| {}) };

    html! {
        <div class="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
            <div class="w-full h-10 mb-2 flex items-center justify-between">
                <div class="flex items-center">
                    <Button style={ButtonStyle::Secondary} width="w-max" on_click={toggle_menu}><MaterialSymbol name="menu"/></Button>
                    <a class="text-2xl pl-4 pr-2">{"amazingCal"}</a>
                    <div class="flex px-2">
                        <div class={CLICKABLE_ELEMENT_CLASS} onclick={backward_one_month}><MaterialSymbol name="arrow_back_ios_new"/></div>
                        <div class={CLICKABLE_ELEMENT_CLASS} onclick={foward_one_month}><MaterialSymbol name="arrow_forward_ios"/></div>
                    </div>
                    <a class="text-2xl">{format!("{}, {:?}", get_month_name(*month), *year)}</a>
                </div>
                <div class="flex items-center">
                    <DropDown open={open.clone()} id="View selector" minimal={false} element={html!{&*view}} options={view_options} return_index={handle_view_menu}/>
                    <DropDown open={open.clone()} id="Account" minimal={false} element={html!{<svg width="40" height="40" data-jdenticon-value={(*email).clone()}></svg>}} options={vec![html!{"Sign out"}]} return_index={handle_account_menu}/>
                </div>
            </div>
            <div class="flex h-full">
                <div class={format!("flex flex-col {} h-full mr-2", if *menu {"w-60"} else {"w-15"})}>
                    <Button style={ButtonStyle::Secondary} width="w-max" on_click={&empty_callback}>
                        <MaterialSymbol name="add"/>
                        {if *menu {html!{{"Event"}}} else {html!{""}}}
                    </Button>
                    {if *menu {
                        html! {
                            <div>
                                <div class="flex justify-between items-center my-2">
                                    <p>{"Calendars"}</p>
                                    <div class={format!("flex items-center {}", CLICKABLE_ELEMENT_CLASS)}>
                                        <a onclick={add_calendar}><MaterialSymbol name="add"/></a>
                                    </div>
                                </div>
                                {
                                    calendars_clone.iter().map(|calendar| html! {
                                       <a>{&*calendar.name}</a>
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } else {html! {}}}
                </div>
                <CalendarView view={view.clone()} month={month.clone()} year={year.clone()}/>
            </div>
            // modals go here
        </div>
    }
}
