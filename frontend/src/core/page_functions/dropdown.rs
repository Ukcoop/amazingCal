use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, DomRectReadOnly, Element, Node, Window};
use yew::{MouseEvent, NodeRef, UseStateHandle};

fn get_window() -> Option<Window> {
    web_sys::window()
}

fn get_document() -> Option<Document> {
    get_window()?.document()
}

fn get_bounding_client_rect(element: &Element) -> Result<DomRectReadOnly, JsValue> {
    let func = js_sys::Reflect::get(element, &JsValue::from_str("getBoundingClientRect"))
        .map_err(|err| {
            JsValue::from_str(&format!(
                "element should have getBoundingClientRect: {:?}",
                err
            ))
        })?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| JsValue::from_str("getBoundingClientRect should be a function"))?;

    let rect_js = func
        .call0(element)
        .map_err(|_| JsValue::from_str("failed to call getBoundingClientRect"))?;

    rect_js
        .dyn_into::<DomRectReadOnly>()
        .map_err(|_| JsValue::from_str("getBoundingClientRect should return a DomRectReadOnly"))
}

pub fn open_menu(
    dropdown_ref: NodeRef,
    button_ref: NodeRef,
    set_open: UseStateHandle<String>,
    id: String,
) -> Box<dyn FnOnce()> {
    let Some(document) = get_document() else {
        return Box::new(|| ());
    };

    let listener = EventListener::new(&document, "mousedown", move |event| {
        let Some(event) = event.dyn_ref::<MouseEvent>() else {
            return;
        };
        let Some(target) = event.target() else {
            return;
        };
        let Some(target_node) = target.dyn_ref::<Node>() else {
            return;
        };

        if id != *set_open {
            return;
        }

        let (Some(dropdown), Some(button)) =
            (dropdown_ref.cast::<Element>(), button_ref.cast::<Element>())
        else {
            return;
        };

        if !dropdown.contains(Some(target_node)) && !button.contains(Some(target_node)) {
            set_open.set("None".to_string());
        }
    });

    return Box::new(move || drop(listener));
}

pub fn position_menu(
    dropdown_ref: NodeRef,
    button_ref: NodeRef,
    dropdown_style: UseStateHandle<(f64, f64)>,
    open: UseStateHandle<String>,
    id: String,
) -> Box<dyn FnOnce()> {
    if id != *open {
        return Box::new(|| ());
    }

    gloo::timers::callback::Timeout::new(0, move || {
        let (Some(dropdown), Some(button)) = (
            dropdown_ref.cast::<web_sys::Element>(),
            button_ref.cast::<web_sys::Element>(),
        ) else {
            return;
        };

        // Handle the Result for button and dropdown
        let button_rect = match get_bounding_client_rect(&button) {
            Ok(rect) => rect,
            Err(_) => return, // handle the error case here
        };

        let dropdown_rect = match get_bounding_client_rect(&dropdown) {
            Ok(rect) => rect,
            Err(_) => return, // handle the error case here
        };

        let Some(window) = get_window() else {
            return;
        };
        let viewport_width = window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let new_top = button_rect.height() + 10.0;
        let mut new_left = button_rect.left();

        let parent_left = match button.parent_element() {
            Some(p) => match get_bounding_client_rect(&p) {
                Ok(rect) => rect.left(),
                Err(_) => 0.0,
            },
            None => 0.0,
        };

        new_left -= parent_left;

        if new_left + dropdown_rect.width() > viewport_width {
            new_left = viewport_width - dropdown_rect.width();
        }

        dropdown_style.set((new_top, new_left));
    })
    .forget();

    return Box::new(|| ());
}
