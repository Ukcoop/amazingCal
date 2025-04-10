use once_cell::sync::Lazy;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use yew::{function_component, html, use_effect_with, use_state, Html, MouseEvent, UseStateHandle};
use yew_router::hooks::use_navigator;

use crate::core::{
    calendar_data::get_todays_date,
    modal_functions::create_event::{new_event, CreateNewEvent},
    page_functions::calendar::{get_current_session, get_user_data, ActiveCalendar},
    shared::Calendar,
};

use crate::components::calendar::month_view::{ContextMenuDeps, MonthView};

use crate::pages::page_components::{
    calendar_menus::CalendarMenus, side_calendar_bar::SideCalendarBar,
    top_calendar_bar::TopCalendarBar,
};

static PUBLIC_SUPABASE_URL: Lazy<&str> =
    Lazy::new(|| option_env!("PUBLIC_SUPABASE_URL").unwrap_or(""));

static PUBLIC_ANON_KEY: Lazy<&str> = Lazy::new(|| option_env!("PUBLIC_ANON_KEY").unwrap_or(""));

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    pub fn init_supabase(supabase_url: String, anon_key: String);
    pub async fn get_session() -> JsValue;
}

#[function_component]
pub fn CalendarPage() -> Html {
    let navigator = use_navigator();
    init_supabase(PUBLIC_SUPABASE_URL.to_string(), PUBLIC_ANON_KEY.to_string());

    let token = use_state(|| "".to_string());

    let calendars: UseStateHandle<Vec<Calendar>> = use_state(Vec::new);
    let active_calendars: UseStateHandle<Vec<ActiveCalendar>> = use_state(Vec::new);
    let selected_calendar = use_state(|| "".to_string());

    let menu = use_state(|| false);
    let modal = use_state(|| "None".to_string());
    let context_menu = use_state(|| "None".to_string());
    let event: UseStateHandle<CreateNewEvent> = use_state(new_event);

    let (today_month, today_year, _) = get_todays_date();
    let month: UseStateHandle<i32> = use_state(|| today_month);
    let year: UseStateHandle<i32> = use_state(|| today_year);

    let token_clone = token.clone();
    let navigator_clone = navigator.clone();
    use_effect_with((), move |_| {
        get_current_session(navigator_clone, token_clone);
    });

    let calendars_clone = calendars.clone();
    let active_calendars_clone = active_calendars.clone();
    let navigator_clone = navigator.clone();
    let token_clone = token.clone();
    use_effect_with(token_clone.clone(), move |_| {
        get_user_data(
            calendars_clone,
            active_calendars_clone,
            navigator_clone,
            token_clone,
        )
    });

    let refresh_data = {
        let calendars = calendars.clone();
        let active_calendars = active_calendars.clone();
        let token_clone = token.clone();

        move |_| {
            get_user_data(
                calendars.clone(),
                active_calendars.clone(),
                navigator.clone(),
                token_clone.clone(),
            );
        }
    };

    let context_menu_clone = context_menu.clone();
    let close_context_menu = move |_: MouseEvent| {
        context_menu_clone.set("None".to_string());
    };

    html! {
        <div onclick={close_context_menu} class="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
            <TopCalendarBar
                menu={menu.clone()}
                month={month.clone()}
                year={year.clone()}
            />
            <div class="flex h-full">
                <SideCalendarBar
                    menu={menu.clone()}
                    context_menu={context_menu.clone()}
                    modal={modal.clone()}
                    calendars={calendars.clone()}
                    active_calendars={active_calendars.clone()}
                    selected_calendar={selected_calendar.clone()}
                />
                <MonthView
                    month={month.clone()}
                    year={year.clone()}
                    modal={modal.clone()}
                    context_menu_deps={ContextMenuDeps {
                        context_menu: context_menu.clone(),
                        modal: modal.clone(),
                        event: event.clone()
                    }}
                    active_calendars={active_calendars.clone()}
                    token={token.clone().to_string()}
                    refresh_data={refresh_data.clone()}
                />
            </div>
            <CalendarMenus
                token={token.clone()}
                modal={modal.clone()}
                selected_calendar={selected_calendar.clone()}
                active_calendars={active_calendars.clone()}
                event={event.clone()}
                refresh_data={refresh_data.clone()}
            />
        </div>
    }
}
