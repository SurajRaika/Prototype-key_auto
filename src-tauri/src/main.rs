// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri;

use std::{
    // sync::mpsc::{self, Receiver, Sender},
    thread,
};

// use lazy_static::lazy_static;
use rdev::listen;
// use std::sync::Mutex;

mod core;
mod types;
use crate::core::core;
// mod command;
use crate::core::input_event_callback;

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            // Spawn a new thread to listen for input events using the callback function.
            thread::spawn(|| {
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
