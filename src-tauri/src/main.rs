
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Just run the app without creating any windows.
    // In lib.rs, we can modify the setup hook to NOT create the main window.
    zerror_lib::run()
}
