use tauri::{AppHandle, Manager};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::SystemTime;
use std::sync::{Arc, Mutex};

use tokio::time::sleep;
use crate::{
    types::{Action, ParallelActions},
    utils::process_parallel_actions,
    ActiveEventKeys, MYREC,MyActivated,
};
use lazy_static::lazy_static;
use rdev::{Event, EventType};

lazy_static! {
    static ref INPUT_EVENT_CHANNEL: (Mutex<Sender<Event>>, Mutex<Receiver<Event>>) = {
        let (sender, receiver) = mpsc::channel();
        (Mutex::new(sender), Mutex::new(receiver))
    };
}

pub fn input_event_callback(event: Event) {
    INPUT_EVENT_CHANNEL
        .0
        .lock()
        .expect("Failed to lock Event_Channel")
        .send(event)
        .expect("Receiver was stopped");
}

fn actions_match(actions1: &[Action], actions2: &[Action]) -> bool {
    if actions1.len() != actions2.len() {
        return false;
    }

    let mut sorted_actions1 = actions1.to_vec();
    let mut sorted_actions2 = actions2.to_vec();

    sorted_actions1.sort_by(|a, b| a.key.cmp(&b.key));
    sorted_actions2.sort_by(|a, b| a.key.cmp(&b.key));

    sorted_actions1
        .iter()
        .zip(sorted_actions2.iter())
        .all(|(a1, a2)| a1.key == a2.key)
}

pub fn core(handle: AppHandle) {
    let counter = Arc::new(Mutex::new(0));

    let initial_system_time = SystemTime::now();
    let receiver = INPUT_EVENT_CHANNEL.1.lock().unwrap();

    let mut buffer: Vec<Event> = vec![];
    let mut previous_event: Option<Event> = None;
    let mut parallel_actions: ParallelActions = vec![];

    for current_event in receiver.iter() {
        if let Some(prev_event) = previous_event.clone() {
            if prev_event.event_type != current_event.event_type {
                match current_event.event_type {
                    EventType::KeyPress(key) => {
                        if !buffer.iter().any(|event| event.event_type == EventType::KeyPress(key)) {
                            buffer.push(current_event.clone());
                        }
                    }
                    EventType::KeyRelease(search_key) => {
                        if let Some(index) = buffer.iter().position(|event| {
                            matches!(event.event_type, EventType::KeyPress(key) | EventType::KeyRelease(key) if key == search_key)
                        }) {
                            parallel_actions.push(Action::new(&buffer[index], &current_event, &initial_system_time));
                            buffer.remove(index);
                            
                            if buffer.is_empty() {
                                println!("\n \n  {:?} \n  \n", parallel_actions);

                                let handle_event_state = &handle.state::<ActiveEventKeys>().0;
                                let event_keys_lock = handle_event_state.lock().unwrap();
                                let event_keys = &*event_keys_lock;
                                
                                for event_key in event_keys {
                                    if actions_match(&event_key.event[0], &parallel_actions) {
                                        println!("Actions match!");
                                        println!("Counter before async task: {}", *counter.lock().unwrap());

                                        if *counter.lock().unwrap() != 0 {
                                            break;
                                        }

                                        let cloned_actions = event_key.actions.clone();
                                        let initial_action_time = event_key.actions[0][0].start_time;

                                        println!("Add");

                                        for actions in cloned_actions {
                                            let counter_clone = Arc::clone(&counter);
                                            tauri::async_runtime::spawn(async move {
                                                {
                                                    let mut counter = counter_clone.lock().unwrap();
                                                    *counter += 1;
                                                    println!("Add Counter: {:?}", *counter);
                                                }

                                                let delay = actions[0].start_time.saturating_sub(initial_action_time);
                                                sleep(delay).await;
                                                process_parallel_actions(actions).await;

                                                let mut counter = counter_clone.lock().unwrap();
                                                *counter -= 1;
                                                println!("Remove Counter: {:?}", *counter);
                                            });
                                        }
                                        
                                        break;
                                    }
                                }




                                let is_recording: bool;
                                let rec = &handle.state::<MYREC>().0;
                                let recording_lock = rec.lock().unwrap();
                                is_recording = *recording_lock;
                                if is_recording {


                                let is_Activated: bool;
                                let aec = &handle.state::<MyActivated>().0;
                                let aec_lock = aec.lock().unwrap();
                                is_Activated = *aec_lock;




                                    let initial_system_time = SystemTime::now();

                                    let json_data = serde_json::to_string(&parallel_actions)
                                        .expect("Failed to serialize actions to JSON");
                                    handle
                                        .emit_all("updateCounter", Some(json_data))
                                        .expect("Failed to send data");
                                }
                                parallel_actions.clear();
                            }
                        }
                    }
                    _ => {}
                };
            }
        } else {
            println!("Previous event is none");
        }

        previous_event = Some(current_event.clone());
    }
}