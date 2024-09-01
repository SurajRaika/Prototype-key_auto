// use std::time::Duration;

use std::time::{Duration, SystemTime};

use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub key: String,
    pub duration: Duration,
    pub start_time: Duration,
}
pub type ParallelActions = Vec<Action>;

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

#[derive(Debug, Deserialize, Clone)]
pub struct EventKey {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub event: Vec<ParallelActions>,
    pub actions: Vec<ParallelActions>,
}

pub type EventKeys = Vec<EventKey>;

// Other types or structs can be defined here as needed
