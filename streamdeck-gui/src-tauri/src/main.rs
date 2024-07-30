// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{borrow::Borrow, sync::Arc};

use streamdeck::{device, Device, StreamDeckMini};
use tauri::async_runtime::Mutex;

#[tauri::command]
fn sd_set_brightness(brightness: u8) {
    let device = StreamDeckMini::new();
    device.set_brightness(brightness)
}

#[tauri::command]
async fn sd_fade_brightness(start_brightness: u8, end_brightness: u8, duration_ms: u64, steps: u8) {
    let device = StreamDeckMini::new();
    for i in 0..steps {
        let progress = i as f64 / (steps - 1) as f64;
        let brightness = (start_brightness as f64 + progress * (end_brightness as f64 - start_brightness as f64)) as u8;
        device.set_brightness(brightness);
        tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms / steps as u64)).await;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sd_set_brightness])
        .invoke_handler(tauri::generate_handler![sd_fade_brightness])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
