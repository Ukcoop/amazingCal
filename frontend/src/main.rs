use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod core;
mod pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/calendar")]
    Calendar,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    return match routes {
        Route::Home => pages::home::home(),
        Route::Login => html! {<pages::login::Login />},
        Route::Calendar => html! {<pages::calendar::CalendarPage />},
        Route::NotFound => pages::not_found::not_found(),
    };
}

#[function_component(Main)]
fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    };
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
