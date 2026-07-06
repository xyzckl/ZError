// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(feature = "tauri")]
    zerror_lib::run();
    #[cfg(not(feature = "tauri"))]
    println!("To run the headless server, use the zerror_headless binary.");
}
