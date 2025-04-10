use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MaterialSymbolsParams {
    pub name: String,
}

#[function_component]
pub fn MaterialSymbol(props: &MaterialSymbolsParams) -> Html {
    return html! {
    <div class="h-[24px]">
        <span class="material-symbols-rounded">
            {props.name.clone()}
        </span>
    </div>
    };
}
