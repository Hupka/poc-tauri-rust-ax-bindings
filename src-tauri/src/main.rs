#![allow(dead_code)]

use tauri::Manager;

mod callbacks;
mod observer;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Register observer for Tauri App.

            // To observe any other application, go to macOS Activity Monitor -> Storage.
            // This shows the PID for each running application.

            // Observer for a macOS app - e.g. Finder
            // observer::register(Some(8751), app.app_handle());

            // Observer for an Electron app - e.g. Slack/Discord/...
            // observer::register(Some(38774), app.app_handle());
            observer::register(Some(66896), app.app_handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
