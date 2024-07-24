use rdev::{simulate, Button, EventType, Key, SimulateError};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

use crate::types::{Action, EventKey};

pub fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str {
        "Alt" => Some(Key::Alt),
        "AltGr" => Some(Key::AltGr),
        "Backspace" => Some(Key::Backspace),
        "CapsLock" => Some(Key::CapsLock),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "Delete" => Some(Key::Delete),
        "DownArrow" => Some(Key::DownArrow),
        "End" => Some(Key::End),
        "Escape" => Some(Key::Escape),
        "F1" => Some(Key::F1),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "Home" => Some(Key::Home),
        "LeftArrow" => Some(Key::LeftArrow),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Return" => Some(Key::Return),
        "RightArrow" => Some(Key::RightArrow),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "UpArrow" => Some(Key::UpArrow),
        "PrintScreen" => Some(Key::PrintScreen),
        "ScrollLock" => Some(Key::ScrollLock),
        "Pause" => Some(Key::Pause),
        "NumLock" => Some(Key::NumLock),
        "BackQuote" => Some(Key::BackQuote),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        "Num0" => Some(Key::Num0),
        "Minus" => Some(Key::Minus),
        "Equal" => Some(Key::Equal),
        "KeyQ" => Some(Key::KeyQ),
        "KeyW" => Some(Key::KeyW),
        "KeyE" => Some(Key::KeyE),
        "KeyR" => Some(Key::KeyR),
        "KeyT" => Some(Key::KeyT),
        "KeyY" => Some(Key::KeyY),
        "KeyU" => Some(Key::KeyU),
        "KeyI" => Some(Key::KeyI),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "LeftBracket" => Some(Key::LeftBracket),
        "RightBracket" => Some(Key::RightBracket),
        "KeyA" => Some(Key::KeyA),
        "KeyS" => Some(Key::KeyS),
        "KeyD" => Some(Key::KeyD),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "SemiColon" => Some(Key::SemiColon),
        "Quote" => Some(Key::Quote),
        "BackSlash" => Some(Key::BackSlash),
        "IntlBackslash" => Some(Key::IntlBackslash),
        "KeyZ" => Some(Key::KeyZ),
        "KeyX" => Some(Key::KeyX),
        "KeyC" => Some(Key::KeyC),
        "KeyV" => Some(Key::KeyV),
        "KeyB" => Some(Key::KeyB),
        "KeyN" => Some(Key::KeyN),
        "KeyM" => Some(Key::KeyM),
        "Comma" => Some(Key::Comma),
        "Dot" => Some(Key::Dot),
        "Slash" => Some(Key::Slash),
        "Insert" => Some(Key::Insert),
        "KpReturn" => Some(Key::KpReturn),
        "KpMinus" => Some(Key::KpMinus),
        "KpPlus" => Some(Key::KpPlus),
        "KpMultiply" => Some(Key::KpMultiply),
        "KpDivide" => Some(Key::KpDivide),
        "Kp0" => Some(Key::Kp0),
        "Kp1" => Some(Key::Kp1),
        "Kp2" => Some(Key::Kp2),
        "Kp3" => Some(Key::Kp3),
        "Kp4" => Some(Key::Kp4),
        "Kp5" => Some(Key::Kp5),
        "Kp6" => Some(Key::Kp6),
        "Kp7" => Some(Key::Kp7),
        "Kp8" => Some(Key::Kp8),
        "Kp9" => Some(Key::Kp9),
        "KpDelete" => Some(Key::KpDelete),
        "Function" => Some(Key::Function),
        _ => {
            if let Ok(num) = key_str.parse::<u32>() {
                Some(Key::Unknown(num))
            } else {
                None
            }
        }
    }
}

/// Simulates a key press or release event with a short delay.
pub async fn send(event_type: &EventType) {
    let delay = Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => eprintln!("Error sending event: {:?}", event_type),
    }
    sleep(delay).await;
}

/// Processes a single action
pub async fn process_action(action: Action, initial_start_time: Duration) {
    let delay = action.start_time.saturating_sub(initial_start_time);
    sleep(delay).await;

    println!("Start key {}", action.key);
    let key_var = string_to_key(&action.key).expect("Error in handling key parsing");
    // Here you would call `send()` to simulate key press
    send(&EventType::KeyPress(key_var)).await;

    sleep(action.duration).await;

    println!("End key {}", action.key);
    // Here you would call `send()` to simulate key release
    send(&EventType::KeyRelease(key_var)).await;
}

/// Processes a set of parallel actions
pub async fn process_parallel_actions(mut parallel_actions: Vec<Action>) {
    parallel_actions.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    let initial_start_time = parallel_actions[0].start_time;

    for action in parallel_actions {
        tauri::async_runtime::spawn(process_action(action, initial_start_time));
    }
}

/// Reads and parses the EventKeys JSON file
pub fn read_event_keys(path: &Path) -> Result<Vec<EventKey>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}
