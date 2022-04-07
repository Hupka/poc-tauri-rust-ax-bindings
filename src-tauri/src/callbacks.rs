use std::{ffi::c_void, mem};

use accessibility::{AXAttribute, AXObserver, AXUIElement};
use accessibility_sys::{AXObserverRef, AXUIElementRef};
use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};

use colored::*;

use crate::utils::TauriState;

/// Entry callback function that is being called by the operating system every time
/// one of the registered notifications is received.
pub unsafe extern "C" fn callback_app_ax_notifications(
    observer: AXObserverRef,
    element: AXUIElementRef,
    notification: CFStringRef,
    context: *mut c_void,
) {
    let _observer: AXObserver = TCFType::wrap_under_get_rule(observer);
    let element: AXUIElement = TCFType::wrap_under_get_rule(element);
    let notification = CFString::wrap_under_get_rule(notification);
    let _context: *mut TauriState = mem::transmute(context);

    match notification.to_string().as_str() {
        other => {
            let role = element.attribute(&AXAttribute::role()).unwrap();
            let pid = element.pid().unwrap();
            println!(
                "PID: {}, event: {}, role: {}, element: {}",
                pid.to_string().bold().blue(),
                other.to_string().bold().red(),
                format!("{:?}", role).green(),
                format!("{:?}", element).yellow(),
            )
        }
    }
}
