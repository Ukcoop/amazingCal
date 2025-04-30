use yew::{html, Html};

use crate::components::main::link::{Link, LinkStyle};

pub fn home() -> Html {
    return html! {
        <div class="flex flex-col items-center mt-40 lg:mt-0 lg:justify-center p-5 w-full h-screen max-h-screen bg-white dark:bg-gray-950">
            <div class="mt-5 flex flex-col justify-center w-max">
                <a class="text-6xl mb-2">{"Welcome to amazingCal"}</a>
                <a class="text-3xl lg:text-lg">{"The minimalist calendar"}</a>
                <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
                <div class="flex lg:flex-row flex-col mt-2">
                    <Link text="login" style={LinkStyle::Secondary} href="/login"/>
                    <Link text="calendar" style={LinkStyle::Primary} href="/calendar"/>
                </div>
            </div>
        </div>
    };
}
