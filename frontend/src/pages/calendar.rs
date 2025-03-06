use yew::{html, Html};

pub fn calendar() -> Html {
    return html! {
        <div class="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
            <a class="text-2xl">{"This will be the calendar."}</a>
        </div>
    };
}
