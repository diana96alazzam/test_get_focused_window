// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};

// The issue does not appear in the following cases:
//      - If I make  decorations true for the window.
//      - If I remove window.is_maximized()
//      - If I remove app.get_focused_window()
//      - If I add delay before I call app.get_focused_window()

fn main() {
    let builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![test_cmd]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _| {});
}

#[tauri::command]
async fn test_cmd(app: AppHandle) -> Result<(), ()> {
    let main_window = app.get_window("main");

    if let Some(window) = main_window {
        let _ = window.is_maximized();
    };

    app.get_focused_window();

    Ok(())
}
