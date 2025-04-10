use yew::{function_component, html, use_state, Callback, Html, MouseEvent, Properties};

use super::material_symbols::MaterialSymbol;

#[derive(PartialEq, Properties)]
pub struct CheckBoxParams {
    pub start_state: bool,
    pub name: String,
    pub uuid: String,
    pub toggle: Callback<(String, String)>,
}

#[function_component]
pub fn CheckBox(props: &CheckBoxParams) -> Html {
    let checked = use_state(|| props.start_state);

    let on_click = {
        let checked = checked.clone();
        let toggle = props.toggle.clone();
        let name = props.name.clone();
        let uuid = props.uuid.clone();
        Callback::from(move |_event: MouseEvent| {
            checked.set(!*checked);
            toggle.emit((name.clone(), uuid.clone()));
        })
    };

    return html! {
        <div onclick={on_click}>
            <MaterialSymbol name={if *checked { "check_box" } else { "check_box_outline_blank" }}/>
        </div>
    };
}
