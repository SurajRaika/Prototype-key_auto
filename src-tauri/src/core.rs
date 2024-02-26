use tauri::{ AppHandle, Manager };

use std::{ sync::mpsc::{ self, Receiver, Sender }, time::SystemTime };

use crate::{ types::Action, MYREC };
use lazy_static::lazy_static;
use rdev::{ Event, EventType };
use std::sync::Mutex;
// crate::types

// Define a lazy static variable for a channel to send and receive events.
lazy_static! {
    static ref INPUT_EVENT_CHANNEL: (Mutex<Sender<Event>>, Mutex<Receiver<Event>>) = {
        let (sender, receiver) = mpsc::channel();
        (Mutex::new(sender), Mutex::new(receiver))
    };
}
// Callback function to be called when an input event is captured.
pub fn input_event_callback(event: Event) {
    // Send the captured event to the channel.
    INPUT_EVENT_CHANNEL.0.lock()
        .expect("Failed to lock Event_Channel")
        .send(event)
        .expect("Receiver was stopped");
}
pub fn core(handle: AppHandle) {
    let intial_system_time = SystemTime::now();
    // Lock the receiver from the event channel.
    let receiver = INPUT_EVENT_CHANNEL.1.lock().unwrap();

    let mut buffer: Vec<Event> = vec![];
    // Initialize variables to keep track of previous events.
    let mut previous_event: Option<Event> = None;

    let mut parallel_actions: Vec<Action> = vec![];
    // let mut user_action_list: Vec<Vec<Action>> = vec![];

    // Iterate over received events from the channel.
    for current_event in receiver.iter() {
        if let Some(prev_event) = previous_event.clone() {
            // Compare the current event with the previous one.
            if prev_event.event_type == current_event.event_type {
            } else {
                // Process the event based on its type.
                match current_event.event_type {
                    EventType::MouseMove { .. } => {/* Ignore mouse move events */}
                    EventType::KeyPress(key) => {
                        // Check if the buffer already contains an event with the same EventType.
                        if !buffer.iter().any(|event| event.event_type == EventType::KeyPress(key)) {
                            // If not, push the current_event into the buffer.
                            buffer.push(current_event.clone());
                        }
                    }
                    EventType::Wheel { .. } => {/* Handle wheel events if needed */}
                    EventType::KeyRelease(search_key) => {
                        if
                            let Some(index) = buffer
                                .iter()
                                .position(|event| {
                                    matches!(
                                event.event_type,
                                EventType::KeyPress(key) | EventType::KeyRelease(key) if key == search_key
                            )
                                })
                        {
                            parallel_actions.push(
                                Action::new(&buffer[index], &current_event, &intial_system_time)
                            );
                            buffer.remove(index);
                            if buffer.is_empty() {
                                println!("\n {:?}", parallel_actions);

                                let app_data_dir=handle.path_resolver().app_data_dir().expect("Failed to read app_daata_dir");
                                let app_json_path=app_data_dir.join("EventKeys.json");
                                
                            //    tauri::generate_context!()
                            //    handle.path_resolver()






                                // Recording Switch
                                let is_recording: bool;
                                let rec = &handle.state::<MYREC>().0;
                                let recording_lock = rec.lock().unwrap();
                                is_recording = *recording_lock;
                                if is_recording {

                                    let json_data = serde_json
                                    ::to_string(&parallel_actions)
                                    .expect("Failed to serialize actions to JSON");
                                
                                    // Emit Keys 
                                    handle
                                        .emit_all("updateCounter", Some(json_data))
                                        .expect("Failed to send data");
                                }

                                parallel_actions.clear();
                            }
                        }
                    }
                    | EventType::ButtonPress(_)
                    | EventType::ButtonRelease(
                          _,
                      ) => {/* Handle button press or release events if needed */}
                };
            }
        } else {
            println!("Previous event is none");
        }

        // Update the previous event for the next iteration.
        previous_event = Some(current_event.clone());
    }
}
