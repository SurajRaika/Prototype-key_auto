// use std::time::Duration;

use std::time::{Duration, SystemTime};

use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Action {
    pub key: String,
    pub duration: Duration,
    pub start_time: Duration,
}

impl Action {
    pub fn new(
        key_press_event: &Event,
        key_release_event: &Event,
        intial_system_time: &SystemTime,
    ) -> Self {
        let start_time = key_press_event
            .time
            .duration_since(*intial_system_time)
            .unwrap();
        let key = match key_press_event.event_type {
            EventType::KeyPress(key) => format!("{:?}", key),
            _ => unimplemented!(),
        };
        let duration = key_release_event
            .time
            .duration_since(key_press_event.time)
            .unwrap();
        Self {
            key,
            duration,
            start_time,
        }
    }
}

// Other types or structs can be defined here as needed
