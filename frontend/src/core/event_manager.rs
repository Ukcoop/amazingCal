use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::shared::Event;

type EventsObject = HashMap<String, Vec<Event>>;
type EventToCalendarMap = HashMap<String, String>;

lazy_static! {
    static ref EVENT_DISPLAY_MANAGER: Mutex<EventDisplayManager> =
        Mutex::new(EventDisplayManager::new());
}

#[derive(PartialEq, Clone)]
pub struct EventDisplayManager {
    events: EventsObject,
    event_to_calendar_map: EventToCalendarMap,
}

impl EventDisplayManager {
    fn new() -> Self {
        EventDisplayManager {
            events: HashMap::new(),
            event_to_calendar_map: HashMap::new(),
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

        web_sys::console::log_1(&event_key.clone().into());

        self.event_to_calendar_map
            .insert(event.uuid.clone(), calendar);

        self.events.entry(event_key).or_default().push(event);
    }

    pub fn edit_event(&mut self, old_key: String, event: Event) {
        let caledar_name = match self.event_to_calendar_map.get(&event.uuid) {
            Some(calendar) => calendar,
            _ => return,
        };

        let new_key = format!(
            "{}-{}-{}-{}",
            caledar_name, event.start.year, event.start.month, event.start.day
        );

        if let Some(events) = self.events.get_mut(&old_key) {
            let mut events = std::mem::take(events);
            let mut new_events: Vec<Event> = events
                .clone()
                .into_iter()
                .filter(|new_event| new_event.uuid != event.uuid)
                .collect();
            std::mem::swap(&mut new_events, &mut events);
        }

        self.events.entry(new_key).or_default().push(event);
    }

    pub fn delete_event(&mut self, old_key: String, uuid: String) {
        if let Some(events) = self.events.get_mut(&old_key) {
            let mut events = std::mem::take(events);
            let mut new_events: Vec<Event> = events
                .clone()
                .into_iter()
                .filter(|new_event| new_event.uuid != uuid)
                .collect();
            std::mem::swap(&mut new_events, &mut events);
        }
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
