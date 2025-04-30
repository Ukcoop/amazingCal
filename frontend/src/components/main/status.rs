use yew::{function_component, html, Html, Properties, UseStateHandle};

use crate::components::main::material_symbols::MaterialSymbol;

#[derive(PartialEq)]
pub enum StatusCode {
    Ok,
    Loading,
    Success,
    Error,
}

#[derive(PartialEq)]
pub struct StatusObject {
    pub code: StatusCode,
    pub data: String,
}

#[derive(Properties, PartialEq)]
pub struct StatusParams {
    pub status: UseStateHandle<StatusObject>,
}

#[function_component]
pub fn Status(props: &StatusParams) -> Html {
    let base = "flex items-center w-full min-h-10 rounded-md mb-1 p-2 text-3xl lg:text-base";

    let rendered_lines = props
        .status
        .data
        .split('\n')
        .enumerate()
        .map(|(index, line)| {
            html! {
                <a key={index.to_string()}>{ line }</a>
            }
        })
        .collect::<Html>();

    return match props.status.code {
        StatusCode::Ok => html! {},
        StatusCode::Loading => html! {
            <div class={format!("{} bg-blue-500/30 border-2 border-blue-500 text-blue-600 dark:text-blue-400 py-4 lg:py-2", base)}>
                <MaterialSymbol name="hourglass"/>
                <div class="flex flex-col">{ rendered_lines }</div>
            </div>
        },
        StatusCode::Success => html! {
            <div class={format!("{} bg-green-500/30 border-2 border-green-500 text-green-600 dark:text-green-400 py-4 lg:py-2", base)}>
                <MaterialSymbol name="check"/>
                <div class="flex flex-col">{ rendered_lines }</div>
            </div>
        },
        StatusCode::Error => html! {
            <div class={format!("{} bg-red-500/30 border-2 border-red-500 text-red-600 dark:text-red-400 py-4 lg:py-2", base)}>
                <MaterialSymbol name="close"/>
                <div class="flex flex-col">{ rendered_lines }</div>
            </div>
        },
    };
}
