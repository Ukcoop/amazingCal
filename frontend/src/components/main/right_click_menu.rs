use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct RightClickMenuProps {
    pub options: Vec<Html>,
    pub return_index: Callback<usize>,
    pub x: i32,
    pub y: i32,
}

#[function_component]
pub fn RightClickMenu(props: &RightClickMenuProps) -> Html {
    let options_container_style = if props.options.len() > 8 {
        "max-height: 200px; overflow-y: auto;"
    } else {
        ""
    };

    html! {
        <div class="bg-white dark:bg-gray-800 border border-black dark:border-gray-700 text-white rounded-md shadow-lg" style={format!("position: fixed; left: {}px; top: {}px;", props.x, props.y)}>
            <div class="flex min-w-20 flex-col items-center" style={options_container_style}>
                {
                    props.options.iter().enumerate().map(|(index, option)| {
                        let not_last = index < props.options.len() - 1;
                        let return_index = props.return_index.clone();
                        let onclick = Callback::from(move |_| {
                            return_index.emit(index);
                        });

                        html! {
                                <div
                                key={format!("option-{}", index)}
                                {onclick}
                                class={classes!(
                                    "flex",
                                    "items-center",
                                    "justify-center",
                                    "w-full",
                                    "px-2",
                                    "py-1",
                                    if not_last { "border border-transparent border-b-black dark:border-b-gray-700" } else { "" }
                                )}
                            >
                                <div class="text-black dark:text-white">
                                    { option.clone() }
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
