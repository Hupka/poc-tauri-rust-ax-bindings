#![allow(dead_code)]

use std::ops::Deref;

use tauri::Manager;

use cocoa::{
    appkit::{
        NSApp, NSEvent, NSEventType, NSWindow, NSWindowOrderingMode, NSWindowTitleVisibility,
    },
    base::{id, BOOL, NO, YES},
};
use objc::{msg_send, sel, sel_impl};

use dispatch::Queue;

mod callbacks;
mod observer;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Register observer for Tauri App.
            // observer::register(None, app.app_handle());

            // To observe any other application, go to macOS Activity Monitor -> Storage.
            // This shows the PID for each running application.

            // Observer for a macOS app - e.g. Finder
            observer::register(Some(19012), app.app_handle());

            // Observer for an Electron app - e.g. Slack/Discord/...
            // observer::register(Some(38774), app.app_handle());
            // observer::register(Some(26096), app.app_handle());

            let window = app.handle().get_window(&"main".to_string()).unwrap();

            if let Ok(www) = window.ns_window() {
                unsafe {
                    setTitlebarAppearsTransparent_(www as id, false);
                }

                unsafe {
                    setTitleVisibility_(www as id, NSWindowTitleVisibility::NSWindowTitleVisible);
                }

                unsafe {
                    set_ignore_mouse_events(www as id, true);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

unsafe fn setMovableByWindowBackground_(www: id, movableByWindowBackground: BOOL) {
    msg_send![www, setMovableByWindowBackground: movableByWindowBackground]
}

unsafe fn setTitlebarAppearsTransparent_(www: id, transparent: BOOL) {
    msg_send![www, setTitlebarAppearsTransparent: transparent]
}

unsafe fn setTitleVisibility_(www: id, visibility: NSWindowTitleVisibility) {
    msg_send![www, setTitleVisibility: visibility]
}

unsafe fn setMovable_(www: id, movable: BOOL) {
    msg_send![www, setMovable: movable]
}

struct MainThreadSafe<T>(T);
unsafe impl<T> Send for MainThreadSafe<T> {}
impl<T> Deref for MainThreadSafe<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

unsafe fn set_ignore_mouse_events(ns_window: id, ignore: bool) {
    let ns_window = MainThreadSafe(ns_window);
    Queue::main().exec_async(move || {
        ns_window.setIgnoresMouseEvents_(if ignore { YES } else { NO });
    });
}
