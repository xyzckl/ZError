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

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let config_dir = exe_dir.join("config");

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        println!("⚠️ 创建 config 目录失败: {}", e);
    }

    let config_path = config_dir.join("config.json");
    if !config_path.exists() {
        let default_config = serde_json::json!({
            "adminToken": "admin"
        });
        if let Err(e) = std::fs::write(&config_path, serde_json::to_string_pretty(&default_config).unwrap()) {
            println!("⚠️ 创建 config.json 失败: {}", e);
        }
    }

    let model_config_path = config_dir.join("model_config.json");
    if !model_config_path.exists() {
        if let Err(e) = std::fs::write(&model_config_path, "{}") {
            println!("⚠️ 创建 model_config.json 失败: {}", e);
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // We will call the start_server logic directly
        let mut port = 3000;
        let mut bind_address = "0.0.0.0".to_string();

        let state = std::sync::Arc::new(ServerState::default());
        let _ = crate::server::start_server(port, bind_address.clone(), state.clone()).await;

        println!("✅ 应用启动完成，Web服务器已启动");

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        {
            let mut restart_tx = crate::server::RESTART_TX.lock().unwrap();
            *restart_tx = Some(tx);
        }

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
                Some((new_port, new_bind)) = rx.recv() => {
                    println!("🔄 重新启动服务器，端口：{}，地址：{}", new_port, new_bind);
                    let _ = crate::server::stop_server(state.clone()).await;
                    port = new_port;
                    bind_address = new_bind;
                    let _ = crate::server::start_server(port, bind_address.clone(), state.clone()).await;
                }
            }
        }
    });
}