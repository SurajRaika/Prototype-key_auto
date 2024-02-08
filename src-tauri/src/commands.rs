use tauri::State;

use crate::MYREC;
use std::sync::{Arc, Mutex};
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
