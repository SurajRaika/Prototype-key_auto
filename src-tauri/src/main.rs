// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
// use tauri::Manager;
use tauri::{
    
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

use std::{ sync::mpsc::{ self, Receiver, Sender }, thread, time::{ Duration, SystemTime } };
// use tokio::task::LocalSet;

use lazy_static::lazy_static;
use rdev::{ listen, Event, EventType, Key };
use std::sync::Mutex;



#[derive(Default)]
struct AppState {
    counter: u32,
}


// use tokio::net::{TcpListener, TcpStream};
// use tokio_tungstenite;
// Define a lazy static variable for a channel to send and receive events.
lazy_static! {
    static ref INPUT_EVENT_CHANNEL: (Mutex<Sender<Event>>, Mutex<Receiver<Event>>) = {
        let (sender, receiver) = mpsc::channel();
        (Mutex::new(sender), Mutex::new(receiver))
    };
    static ref INPUT_EVENT_CHANNEL_2:  (Mutex<mpsc::Sender<String>>, Mutex<mpsc::Receiver<String>>) = {
    let (sender, receiver) = mpsc::channel();
    (Mutex::new(sender), Mutex::new(receiver))
};}

#[derive(Debug, Clone)]
struct Action {
    key: Key,
    duration: Duration,
    start_time: Duration,
}

impl Action {
    fn new(
        key_press_event: &Event,
        key_release_event: &Event,
        intial_system_time: &SystemTime
    ) -> Self {
        let start_time = key_press_event.time.duration_since(*intial_system_time).unwrap();
        let key = match key_press_event.event_type {
            EventType::KeyPress(key) => key,
            _ => unimplemented!(),
        };
        let duration = key_release_event.time.duration_since(key_press_event.time).unwrap();
        Self {
            key,
            duration,
            start_time,
        }
    }
}

// Callback function to be called when an input event is captured.
fn input_event_callback(event: Event) {
    // Send the captured event to the channel.
    INPUT_EVENT_CHANNEL.0.lock()
        .expect("Failed to lock Event_Channel")
        .send(event)
        .expect("Receiver was stopped");
}


#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}


#[tauri::command]
async fn greet<R: Runtime>(
    app: AppHandle<R>,name: &str)-> Result<(),String> {
    // Use tokio to run an asynchronous task that sends data to the frontend every 2 seconds
    tauri::async_runtime::spawn(async move {
        let mut counter=0;
        let receiver = INPUT_EVENT_CHANNEL_2.1.lock().unwrap();
        for current_event in receiver.iter() {
            app.emit_all("updateCounter", Some(current_event)).expect("Failed to send data");

        }
        
    });
    Ok(())
}


fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Spawn a new thread to listen for input events using the callback function.
            thread::spawn(|| {
                if let Err(error) = listen(input_event_callback) {
                    println!("Error: {:?}", error)
                }
                
            });
            thread::spawn(|| {
            let intial_system_time = SystemTime::now();
            // Lock the receiver from the event channel.
            let receiver = INPUT_EVENT_CHANNEL.1.lock().unwrap();

            let mut buffer: Vec<Event> = vec![];
            // Initialize variables to keep track of previous events.
            let mut previous_event: Option<Event> = None;

            let mut parallel_actions: Vec<Action> = vec![];
            let mut user_action_list: Vec<Vec<Action>> = vec![];

            // Iterate over received events from the channel.
            for current_event in receiver.iter() {
                if let Some(prev_event) = previous_event.clone() {
                    // Compare the current event with the previous one.
                    if prev_event.event_type == current_event.event_type {
                        // println!("Same");
                    } else {
                        // println!("Different");

                        // Process the event based on its type.
                        match current_event.event_type {
                            EventType::MouseMove { .. } => {/* Ignore mouse move events */}
                            EventType::KeyPress(key) => {
                                // Check if the buffer already contains an event with the same EventType.
                                if
                                    !buffer
                                        .iter()
                                        .any(|event| event.event_type == EventType::KeyPress(key))
                                {
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
                                        Action::new(
                                            &buffer[index],
                                            &current_event,
                                            &intial_system_time
                                        )
                                    );
                                    buffer.remove(index);
                                    // println!("{:?} {}",buffer,buffer.len());
                                    if buffer.is_empty() {
                                        println!("\n {:?}", parallel_actions);

                                        // app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
                                        // user_action_list.push(parallel_actions.clone());

                                        // app.emit_all(
                                        //     "event-name",
                                        //     format!("{:?}", parallel_actions)
                                        // ).unwrap();
                                        INPUT_EVENT_CHANNEL_2.0.lock()
                                        .expect("Failed to lock Event_Channel")
                                        .send(format!("{:?}", parallel_actions))
                                        .expect("Receiver was stopped");

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
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
