use once_cell::sync::Lazy;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;

use yew::{function_component, html, use_effect_with, use_state, Html};
use yew_router::hooks::use_navigator;

use crate::Route;

static PUBLIC_SUPABASE_URL: Lazy<&str> =
    Lazy::new(|| option_env!("PUBLIC_SUPABASE_URL").unwrap_or(""));
static PUBLIC_ANON_KEY: Lazy<&str> = Lazy::new(|| option_env!("PUBLIC_ANON_KEY").unwrap_or(""));

#[wasm_bindgen(module = "/src/js/auth_handler.js")]
extern "C" {
    pub fn init_supabase(supabase_url: String, anon_key: String);
    pub async fn get_session() -> JsValue;
}

#[function_component]
pub fn Calendar() -> Html {
    let navigator = use_navigator();
    init_supabase(PUBLIC_SUPABASE_URL.to_string(), PUBLIC_ANON_KEY.to_string());

    let token = use_state(|| "".to_string());
    let token_clone = token.clone();

    use_effect_with((), move |_| {
        spawn_local(async move {
            let new_token = get_session().await.as_string().unwrap_or_default();

            if new_token == *"" {
                if let Some(navigator) = &navigator {
                    navigator.push(&Route::Calendar);
                }
            } else {
                token_clone.set(new_token);
            }
        });
        || ()
    });

    html! {
        <div class="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
            <a class="text-2xl">{ (*token).clone() }</a>
        </div>
    }
}
