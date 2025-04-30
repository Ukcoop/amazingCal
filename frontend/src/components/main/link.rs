use yew::{function_component, html, Html, Properties};

#[derive(PartialEq)]
pub enum LinkStyle {
    Primary,
    Secondary,
}

#[derive(Properties, PartialEq)]
pub struct LinkParams {
    pub text: String,
    pub style: LinkStyle,
    pub href: String,
}

#[function_component]
pub fn Link(props: &LinkParams) -> Html {
    let applied_style = match props.style {
        LinkStyle::Primary => "flex items-center justify-center w-full lg:w-max h-20 lg:h-10 ml-2 mb-4 lg:mb-0 text-3xl lg:text-base bg-black dark:bg-white hover:bg-gray-900 hover:dark:bg-gray-300 text-white dark:text-black rounded-md p-2 px-4".to_string(),
        LinkStyle::Secondary => "flex items-center justify-center w-full lg:w-max h-20 lg:h-10 ml-2 mb-4 lg:mb-0 text-3xl lg:text-base bg-transparent border-2 border-black dark:border-gray-500 hover:bg-gray-200 hover:dark:bg-gray-900 text-black dark:text-white rounded-md p-2 px-4".to_string()
    };

    return html! {
        <a class={applied_style} href={props.href.clone()}>{props.text.clone()}</a>
    };
}
