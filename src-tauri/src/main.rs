#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;
use std::{ffi::c_void, mem};

use accessibility::{AXObserver, AXUIElement};
use accessibility_sys::{kAXFocusedUIElementChangedNotification, AXObserverRef, AXUIElementRef};
use core_foundation::date::{CFAbsoluteTime, CFDate};
use core_foundation::{dictionary::CFDictionaryRef, runloop::CFRunLoop, string::CFStringRef};
use tauri::api::notification::Notification;
use tauri::window::WindowBuilder;
use tauri::{Manager, WindowUrl};

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let context = Info::new(
        "Adrian".to_string(),
        "Hupka".to_string(),
        33,
        app.app_handle().clone(),
    );

    let now = CFDate::now().abs_time();
    let (elapsed_tx, elapsed_rx) = mpsc::channel();
    let ctx2 = Info2 {
        start_time: now,
        elapsed_tx,
    };

    let copy_handle = app.handle().clone();
    thread::spawn(|| {
        magic(ctx2);
    });

    // thread::spawn(move || );
    loop {
        if let Ok(elapsed) = elapsed_rx.try_recv() {
            println!("wait_200_milliseconds, elapsed: {}", elapsed);
        }
        thread::sleep(std::time::Duration::from_millis(250));
    }

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
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
    // let observer: *mut AXObserver = mem::transmute(raw_info);

    // let info: *mut Info = mem::transmute(raw_info);
    // (*info).print_stuffer();
    // (*info).create_window();

    let info: *mut Info2 = unsafe { mem::transmute(raw_info) };
    let now = CFDate::now().abs_time();
    let elapsed = now - unsafe { (*info).start_time };
    let err = unsafe { (*info).elapsed_tx.send(elapsed) };

    println!("{:?}", err);
}
struct Info {
    first_name: String,
    last_name: String,
    age: u32,
    app_handle: tauri::AppHandle,
}

impl Info {
    fn new(first_name: String, last_name: String, age: u32, app_handle: tauri::AppHandle) -> Self {
        Self {
            first_name,
            last_name,
            age,
            app_handle,
        }
    }

    pub fn print_stuff(&self) {
        println!("{:?}", self.age);
        println!("{:?}", self.first_name);
        println!("{:?}", self.last_name);
    }

    pub fn print_stuffer(&self) {
        println!("asdasda");
    }

    pub fn create_window(&self) {
        WindowBuilder::new(
            &self.app_handle,
            "My man",
            WindowUrl::App("index.html".into()),
        )
        .title("ASDASD".to_string())
        .inner_size(500.0, 300.)
        .center()
        .build()
        .unwrap();
    }
}

struct Info2 {
    start_time: CFAbsoluteTime,
    elapsed_tx: mpsc::Sender<f64>,
}
