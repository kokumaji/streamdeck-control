// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use streamdeck::{device, Device, StreamDeckMini};

#[tauri::command]
async fn sd_set_brightness(brightness: u8) {
    let device = StreamDeckMini::new();
    device.set_brightness(brightness)
}

#[tauri::command]
async fn sd_fade_brightness(brightness: u8) {
    let device = StreamDeckMini::new();
    device.fade_brightness(0, brightness, 2000, 10).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sd_set_brightness, sd_fade_brightness])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
