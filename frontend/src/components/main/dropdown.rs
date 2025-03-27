use yew::prelude::*;

use crate::components::main::material_symbols::MaterialSymbol;

use crate::core::page_functions::dropdown::{open_menu, position_menu};

#[derive(Properties, PartialEq, Clone)]
pub struct DropDownProps {
    pub open: UseStateHandle<String>,
    pub id: String,
    pub minimal: bool,
    pub element: Html,
    pub options: Vec<Html>,
    pub return_index: Callback<usize>,
}

#[function_component(DropDown)]
pub fn drop_down(props: &DropDownProps) -> Html {
    let dropdown_style = use_state(|| (0.0, 0.0));
    let dropdown_ref = use_node_ref();
    let button_ref = use_node_ref();

    let onclick_close = {
        let open = props.open.clone();
        Callback::from(move |_| open.set("None".to_string()))
    };

    let onclick_open = {
        let open = props.open.clone();
        let id = props.id.clone();
        Callback::from(move |_| open.set(id.clone()))
    };

    {
        let dropdown_ref = dropdown_ref.clone();
        let button_ref = button_ref.clone();
        let props = props.clone();
        use_effect_with(props.open.clone(), move |_| {
            open_menu(dropdown_ref, button_ref, props.open, props.id)
        });
    }

    {
        let dropdown_ref = dropdown_ref.clone();
        let button_ref = button_ref.clone();
        let dropdown_style = dropdown_style.clone();
        let open = props.open.clone();
        let id = props.id.clone();

        use_effect_with(open.clone(), move |_| {
            position_menu(dropdown_ref, button_ref, dropdown_style, open, id)
        });
    }

    let options_container_style = if props.options.len() > 8 {
        "max-height: 200px; overflow-y: auto;"
    } else {
        ""
    };

    html! {
        <div class="relative flex flex-col items-center p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-900">
            <div
                ref={button_ref}
                class="flex items-center cursor-pointer"
                onclick={if *props.open == props.id { onclick_close } else { onclick_open }}
            >
                if !props.minimal {
                    if *props.open == props.id {
                        <MaterialSymbol name="expand_less" />
                    } else {
                        <MaterialSymbol name="expand_more" />
                    }
                }
                { props.element.clone() }
            </div>
            if *props.open == props.id {
                <div
                    ref={dropdown_ref}
                    class="absolute z-10 bg-white dark:bg-gray-800 border border-black dark:border-gray-700 text-white rounded-md shadow-lg"
                    style={format!("top: {}px; left: {}px", dropdown_style.0, dropdown_style.1)}
                >
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
        </div>
    }
}
