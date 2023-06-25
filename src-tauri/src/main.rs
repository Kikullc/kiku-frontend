// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![minimize, maximize, close])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn minimize(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn close(window: tauri::Window) {
    window.close().unwrap();
}
