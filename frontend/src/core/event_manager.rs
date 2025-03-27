use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::JsValue;

use crate::core::shared::Event;

type EventsObject = HashMap<String, Vec<Event>>;

lazy_static! {
    static ref EVENT_DISPLAY_MANAGER: Mutex<EventDisplayManager> =
        Mutex::new(EventDisplayManager::new());
    //static ref EVENT_MANAGER: Mutex<EventManager> = Mutex::new(EventManager::new());
}

pub struct EventDisplayManager {
    events: EventsObject,
}

impl EventDisplayManager {
    fn new() -> Self {
        EventDisplayManager {
            events: HashMap::new(),
        }
    }

    pub fn get_instance() -> &'static Mutex<EventDisplayManager> {
        &EVENT_DISPLAY_MANAGER
    }

    pub fn add_event(&mut self, calendar: String, event: Event) {
        let event_key = format!(
            "{}-{}-{}-{}",
            calendar, event.start.year, event.start.month, event.start.day
        );

        web_sys::console::log_1(&JsValue::from_str(&event_key));

        self.events.entry(event_key).or_default().push(event);
    }

    pub fn get_events_by_key(&self, key: &str) -> Vec<Event> {
        return self
            .events
            .get(key)
            .map_or_else(Vec::new, |events| events.clone());
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
/*
pub struct EventManager;

impl EventManager {
    fn new() -> Self {
        EventManager
    }

    pub fn get_instance() -> &'static Mutex<EventManager> {
        &EVENT_MANAGER
    }

    pub fn post_event(&self) {
        println!("Post event");
    }
}
*/
