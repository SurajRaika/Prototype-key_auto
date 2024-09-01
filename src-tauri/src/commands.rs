use std::io::Read;
use tauri::State;

use crate::{types::{EventKey, EventKeys}, ActiveEventKeys, MyActivated, MYREC};
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


// #[derive(Default)]
#[tauri::command]
pub fn Play<'r>(state: State<'r, MyActivated>) {
    let recording = &state.0;
    let mut recording_lock = recording.lock().unwrap();
    *recording_lock = true;
}
#[tauri::command]
pub fn Pause<'r>(state: State<'r, MyActivated>) {
    let recording = &state.0;
    let mut recording_lock = recording.lock().unwrap();
    *recording_lock = false;
}

#[tauri::command]
pub fn add_active_event_key(name: &str, state: State<'_, ActiveEventKeys>) -> Result<(), String> {
    // Parse the JSON string into EventKey
    let event_key: EventKey = serde_json::from_str(name)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
     let recording = &state.0;
    // Acquire the lock on the shared state
    let mut recording_lock = recording.lock()
        .map_err(|_| "Failed to acquire lock on shared state".to_string())?;


    if let Some(pos) = recording_lock.iter().position(|x| x.id == event_key.id) {
        recording_lock.remove(pos);
        return Err("Event key found in active event keys".to_string());
    } else {
            // Add the new event to the vector
    recording_lock.push(event_key);
    }

    Ok(())
}


#[tauri::command]
pub fn remove_active_event_key(name: &str, state: State<'_, ActiveEventKeys>) -> Result<(), String> {
    // Parse the JSON string into EventKey
    let event_key: EventKey = serde_json::from_str(name)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let recording = &state.0;
    // Acquire the lock on the shared state
    let mut recording_lock = recording.lock()
        .map_err(|_| "Failed to acquire lock on shared state".to_string())?;
    // Remove the event from the vector if it exists
    if let Some(pos) = recording_lock.iter().position(|x| x.id == event_key.id) {
        recording_lock.remove(pos);
    } else {
        return Err("Event key not found in active event keys".to_string());
    }
    Ok(())
}


// #[tauri::command]
// pub fn load_keybidings<'r>(state: State<'r, ActiveEventKeys>) {
//     let path = std::path::Path::new("/home/surajraika/.local/share/com.KeyAuto.dev/EventKeys.json");
//     let mut file = File::open(&path).expect("Failed to open EventKeys.json");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .expect("Failed to read file");
//     println!("File contents: {}", contents);
//     let event_key: EventKeys = serde_json::from_str(&contents).expect("Failed to parse JSON");

//     let recording = &state.0;
//     let mut recording_lock = recording.lock().unwrap();
//     *recording_lock = event_key;
// }
