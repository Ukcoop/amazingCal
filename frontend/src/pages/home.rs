use yew::{html, Html};

use crate::components::main::link::{Link, LinkStyle};

pub fn home() -> Html {
    return html! {
        <div class="flex flex-col justify-center ml-64 p-5 w-max h-screen max-h-screen bg-white dark:bg-gray-950">
            <a class="text-4xl">{"Hello there, this is the homepage of amazingCal."}</a>
            <div class="h-0 border dark:border-gray-600 border-black my-2"></div>
            <div class="flex mt-2">
                <Link text="login" style={LinkStyle::Secondary} href="/login"/>
                <Link text="calendar" style={LinkStyle::Primary} href="/calendar"/>
            </div>
        </div>
    };
}
