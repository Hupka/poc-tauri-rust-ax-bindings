#![allow(dead_code)]

use std::thread;
use std::{ffi::c_void, mem};

use accessibility::{AXObserver, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementChangedNotification, AXObserverRef, AXUIElementRef};
use core_foundation::{runloop::CFRunLoop, string::CFStringRef};
use tauri::window::WindowBuilder;
use tauri::{Manager, WindowUrl};

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let context = Info {
        app_handle: app.app_handle().clone(),
    };

    thread::spawn(|| {
        magic(context);
    });

    app.run(|_, _| {});
}

fn magic<T>(ctx: T) {
    let app = dbg!(AXUIElement::application_with_bundle("com.apple.dt.Xcode"));

    if let Ok(app) = app {
        if let Ok(pid) = app.pid() {
            if let Ok(mut observer) = AXObserver::new(pid, callback) {
                // Start runloop
                observer.start();

                // Add notification
                let _ = observer.add_notification(
                    kAXFocusedUIElementChangedNotification.to_string(),
                    &app,
                    ctx,
                );

                CFRunLoop::run_current();
            }
        }
    }
}

unsafe extern "C" fn callback(
    _observer: AXObserverRef,
    _element: AXUIElementRef,
    _notification: CFStringRef,
    raw_info: *mut c_void,
) {
    let info: *mut Info = mem::transmute(raw_info);

    let window = WindowBuilder::new(
        &(*info).app_handle,
        "Window",
        WindowUrl::App("index.html".into()),
    )
    .title("Window opened by AX Notification")
    .inner_size(500.0, 300.)
    .build();
}

struct Info {
    app_handle: tauri::AppHandle,
}
