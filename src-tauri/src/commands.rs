use std::io::Read;
use tauri::State;

use crate::{types::EventKeys, ActiveEventKeys, MYREC , MyActivated};
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
// #[derive(Default)]
#[tauri::command]
pub fn start_record<'r>(state: State<'r, MYREC>) {
    let recording = &state.0;
    let mut recording_lock = recording.lock().unwrap();
    *recording_lock = true;
}
#[tauri::command]
pub fn stop_record<'r>(state: State<'r, MYREC>) {
    let recording = &state.0;
    let mut recording_lock = recording.lock().unwrap();
    *recording_lock = false;
}

#[tauri::command]
pub fn load_keybidings<'r>(state: State<'r, ActiveEventKeys>) {
    let path = std::path::Path::new("/home/surajraika/.local/share/com.KeyAuto.dev/EventKeys.json");
    let mut file = File::open(&path).expect("Failed to open EventKeys.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    println!("File contents: {}", contents);
    let event_key: EventKeys = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let recording = &state.0;
    let mut recording_lock = recording.lock().unwrap();
    *recording_lock = event_key;
}
