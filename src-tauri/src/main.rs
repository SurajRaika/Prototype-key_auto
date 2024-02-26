// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{self, Manager};

use std::sync::{Arc, Mutex};
use std::{
    // sync::mpsc::{self, Receiver, Sender},
    thread,
};

// use lazy_static::lazy_static;
use rdev::listen;
// use std::sync::Mutex;

mod commands;
mod core;
mod types;
use crate::commands::{start_record, stop_record};
use crate::core::core;
use crate::core::input_event_callback;
#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

pub struct MYREC(Arc<Mutex<bool>>);

fn main() {
    // let app_state = AppState::default();

    tauri::Builder::default()
        .device_event_filter(tauri::DeviceEventFilter::Always)
        .invoke_handler(tauri::generate_handler![start_record, stop_record])
        .setup(|app| {
            app.manage(MYREC(Arc::new(Mutex::new(false))));
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
