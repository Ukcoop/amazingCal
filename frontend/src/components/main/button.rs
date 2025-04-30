use yew::{function_component, html, Callback, Children, Html, MouseEvent, Properties};

#[derive(PartialEq)]
pub enum ButtonStyle {
    Primary,
    Secondary,
}

#[derive(Properties, PartialEq)]
pub struct ButtonParams {
    #[prop_or_default]
    pub children: Children,
    pub style: ButtonStyle,
    pub width: String,
    #[prop_or_else(|| Callback::from(|_| ())) ]
    pub on_click: Callback<MouseEvent>,
    #[prop_or("".to_string())]
    pub responsive_hide: String,
}

#[function_component]
pub fn Button(props: &ButtonParams) -> Html {
    let applied_style = match props.style {
        ButtonStyle::Primary => "flex items-center justify-center h-20 lg:h-10 mb-4 lg:mb-1 text-3xl lg:text-base bg-black dark:bg-white hover:bg-gray-900 hover:dark:bg-gray-300 text-white dark:text-black rounded-md p-2 px-4".to_string(),
        ButtonStyle::Secondary => "flex items-center justify-center h-20 lg:h-10 mb-4 lg:mb-1 text-3xl lg:text-base bg-transparent border-2 border-black dark:border-gray-500 hover:bg-gray-200 hover:dark:bg-gray-900 text-black dark:text-white rounded-md p-2 px-4".to_string()
    };

    return html! {
        <div class={format!("{} {}", applied_style, props.responsive_hide)} onclick={props.on_click.clone()}>
            { props.children.clone() }
        </div>
    };
}
