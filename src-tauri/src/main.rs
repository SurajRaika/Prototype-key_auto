// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{self, Manager};
use types::{EventKey, EventKeys};
mod utils;

use std::sync::{Arc, Mutex};
use std::{
    // sync::mpsc::{self, Receiver, Sender},
    thread,
};

// use lazy_static::lazy_static;
use rdev::listen;
// use std::sync::Mutex;
use std::io::Read;

mod commands;
mod core;
mod types;
use crate::commands::{start_record, stop_record};
use crate::core::core;
use crate::core::input_event_callback;

use std::fs::File;

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

pub struct MYREC(Arc<Mutex<bool>>);
pub struct ActiveEventKeys(Arc<Mutex<EventKeys>>);
pub struct MyActivated(Arc<Mutex<bool>>);





fn main() {
    // let app_state = AppState::default();

    tauri::Builder::default()
        .device_event_filter(tauri::DeviceEventFilter::Always)
        .invoke_handler(tauri::generate_handler![start_record, stop_record])
        .setup(|app| {
            let path = std::path::Path::new(
                "/home/surajraika/.local/share/com.KeyAuto.dev/EventKeys.json",
            );
            let mut file = File::open(&path).expect("Failed to open EventKeys.json");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            println!("File contents: {}", contents);
            let event_key: EventKeys =
                serde_json::from_str(&contents).expect("Failed to parse JSON");

            app.manage(MYREC(Arc::new(Mutex::new(false))));
            app.manage(MyActivated(Arc::new(Mutex::new(false))));
            app.manage(ActiveEventKeys(Arc::new(Mutex::new(event_key))));

            // Spawn a new thread to listen for input events using the callback function.
            tauri::async_runtime::spawn(async move {
                if let Err(error) = listen(input_event_callback) {
                    println!("Error: {:?}", error)
                }
            });
            let handle = app.handle();
            thread::spawn(move || core(handle));
            // println!("Setup Function End");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
