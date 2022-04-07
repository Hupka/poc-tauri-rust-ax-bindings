# Playground app to utilize macOS Accessibility APIs within Tauri Apps.

**How to run this project:**
* Clone repository
* Look up the Process IDs of the apps you want to monitor. This is easiest done using the Activity Monitor -> Storage/Memory tab. There, for each app the PID is listed.
* Add observers in the `main.rs` as part of the tauri app's `setup()` routine.
* execute `yarn tauri dev`
