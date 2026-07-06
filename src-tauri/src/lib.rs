// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 模块声明
pub mod commands;
pub mod database;
pub mod logger;
pub mod server;
pub mod types;
pub mod window_size;

use crate::window_size::{resolve_window_size, MAIN_WINDOW_PRESET};
pub use commands::open_text_window;
pub use commands::{
    clear_request_logs, convert_doc_to_docx, create_directory, fetch_image_as_base64, file_exists,
    get_daily_request_counts, get_request_logs, get_username, greet, open_cache_dir, open_devtools,
    open_url_content_window, read_config, read_doc_range, read_docx_range, read_excel_headers,
    read_excel_range, read_file_bytes, read_file_range, read_file_text, read_model_config,
    request_admin_elevation, segment_text, write_config, write_model_config
};
pub use database::*;
pub use database::{
    clear_folder_questions, delete_folder, delete_question, delete_questions, move_folder,
    rename_folder,
};
pub use server::{get_server_status, start_server, stop_server};
use tauri::Manager;
pub use types::*;

fn show_main_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    #[cfg(target_os = "macos")]
    {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
            let _ = window.set_focus();
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        if let Some(window) = app.get_webview_window("main") {
            if window.is_minimized().unwrap_or(false) {
                let _ = window.unminimize();
            }
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

pub fn run() {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let base_dir = format!("{}/.local/share/zerror", home_dir);
    let db_path = format!("{}/airesponses.db", base_dir);

    if let Err(e) = std::fs::create_dir_all(&base_dir) {
        println!("⚠️ 创建数据目录失败: {}", e);
    } else {
        if let Err(e) = crate::database::init_database_schema(&db_path) {
            println!("⚠️ 初始化数据库失败: {}", e);
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // We will call the start_server logic directly
        let port = 80;
        let bind_address = "0.0.0.0".to_string();

        let state = ServerState::default();
        let _ = crate::server::start_server(port, bind_address, std::sync::Arc::new(state)).await;

        println!("✅ 应用启动完成，Web服务器已启动");
        // Keep the server running
        tokio::signal::ctrl_c().await.unwrap();
    });
}