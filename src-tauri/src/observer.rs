use std::thread;

use accessibility_sys::{
    kAXApplicationActivatedNotification, kAXApplicationDeactivatedNotification,
    kAXApplicationHiddenNotification, kAXApplicationShownNotification, kAXCreatedNotification,
    kAXFocusedUIElementChangedNotification, kAXFocusedWindowChangedNotification,
    kAXMainWindowChangedNotification, kAXMovedNotification, kAXResizedNotification,
    kAXSelectedChildrenChangedNotification, kAXSelectedChildrenMovedNotification,
    kAXUIElementDestroyedNotification, kAXWindowCreatedNotification,
    kAXWindowDeminiaturizedNotification, kAXWindowMiniaturizedNotification,
    kAXWindowMovedNotification, kAXWindowResizedNotification,
};

use accessibility::{AXObserver, AXUIElement};
use core_foundation::runloop::CFRunLoop;

use crate::callbacks::callback_app_ax_notifications;
use crate::utils::TauriState;

static OBSERVER_NOTIFICATIONS: &'static [&'static str] = &[
    kAXFocusedUIElementChangedNotification,
    kAXFocusedWindowChangedNotification,
    kAXApplicationShownNotification,
    kAXApplicationHiddenNotification,
    kAXWindowCreatedNotification,
    kAXMainWindowChangedNotification,
    kAXApplicationDeactivatedNotification,
    kAXApplicationActivatedNotification,
    kAXWindowMovedNotification,
    kAXWindowResizedNotification,
    kAXWindowMiniaturizedNotification,
    kAXWindowDeminiaturizedNotification,
    kAXUIElementDestroyedNotification,
    kAXSelectedChildrenMovedNotification,
    kAXSelectedChildrenChangedNotification,
    kAXResizedNotification,
    kAXMovedNotification,
    kAXCreatedNotification,
];

/// AX Observer
/// ================================
/// This call registers a macOS AXObserver for an application given its PID.
///
/// The list of notifications added to this observer can be modified at the
/// top of the file in a static array.
pub fn register(app_pid: Option<i32>, app_handle: tauri::AppHandle) {
    let handle_move_copy = app_handle.clone();
    thread::spawn(move || {
        let mut pid: i32 = std::process::id().try_into().unwrap();

        if let Some(p) = app_pid {
            pid = p;
        }

        // 1. Create AXObserver
        let app_observer = AXObserver::new(pid, callback_app_ax_notifications);
        let ui_element = AXUIElement::application(pid);

        if let Ok(mut app_observer) = app_observer {
            // 2. Start AXObserver before adding notifications
            app_observer.start();

            // 3. Add notifications
            for notification in OBSERVER_NOTIFICATIONS.iter() {
                let _ = app_observer.add_notification(
                    notification,
                    &ui_element,
                    TauriState {
                        handle: handle_move_copy.clone(),
                    },
                );
            }

            // 4. Kick of RunLoop on this thread
            CFRunLoop::run_current();
        }
    });
}
