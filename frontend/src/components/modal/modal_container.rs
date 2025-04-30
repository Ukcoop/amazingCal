use yew::{function_component, html, Callback, Html, MouseEvent, Properties, UseStateHandle};

use crate::components::main::material_symbols::MaterialSymbol;

#[derive(Properties, PartialEq)]
pub struct ModalContainerParams {
    pub title: String,
    pub component: Html,
    pub modal: UseStateHandle<String>,
}

#[function_component]
pub fn ModalContainer(props: &ModalContainerParams) -> Html {
    // Directly create a callback that stops event propagation.
    let no_propagation = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    let close_modal = {
        let modal_clone = props.modal.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            modal_clone.set("".to_string());
        })
    };

    return html! {
        <div onclick={no_propagation} class="flex items-center justify-center absolute top-0 left-0 w-full h-full bg-black/20 backdrop-blur-sm">
            <div class="w-screen lg:w-auto lg:min-w-80 p-4 lg:rounded-md bg-white dark:bg-gray-800">
                <div class="flex justify-between">
                    <a class="text-4xl lg:text-xl">{props.title.clone()}</a>
                    <div class="p-5 lg:p-0 bg-red-500/20 rounded-md cursor-pointer hover:bg-red-500/40 lg:bg-transparent" onclick={close_modal}>
                        <MaterialSymbol name="close" />
                    </div>
                </div>
                {props.component.clone()}
            </div>
        </div>
    };
}
