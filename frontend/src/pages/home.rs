use yew::{html, Html};

use crate::components::main::link::{Link, LinkStyle};

pub fn home() -> Html {
    return html! {
        <div class="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
            <a class="text-2xl">{"Hello there, this is the homepage of amazingCal."}</a>
            <div class="flex mt-2">
                <Link text="login" style={LinkStyle::Primary} href="/login"/>
                <Link text="calendar" style={LinkStyle::Primary} href="/calendar"/>
            </div>
        </div>
    };
}
