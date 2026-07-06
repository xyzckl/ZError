use std::sync::Arc;
use parking_lot::Mutex;
use zerror_lib::types::{ServerState, ServerInfo};
use zerror_lib::logger::RequestLogger;

#[tokio::main]
async fn main() {
    println!("ZError Headless Server starting...");

    // Setup basic directories
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let base_dir = format!("{}/.local/share/zerror", home_dir);
    let db_path = format!("{}/airesponses.db", base_dir);

    if let Err(e) = std::fs::create_dir_all(&base_dir) {
        println!("⚠️ Failed to create data directory: {}", e);
    } else {
        if let Err(e) = zerror_lib::database::init_database_schema(&db_path) {
            println!("⚠️ Failed to initialize database: {}", e);
        }
    }

    // Initialize state
    let state = Arc::new(ServerState {
        info: Arc::new(Mutex::new(ServerInfo {
            running: false,
            port: None,
            url: None,
        })),
        handle: Arc::new(Mutex::new(None)),
        logger: RequestLogger::default(),
        app_handle: None,
    });

    // Default values for headless. In a real scenario these could come from env or arguments.
    let port = std::env::var("PORT").unwrap_or("1420".to_string()).parse().unwrap_or(1420);
    let bind_address = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());

    match zerror_lib::server::start_server_headless(port, bind_address, state.clone()).await {
        Ok(info) => println!("Server started successfully at {:?}", info.url),
        Err(e) => eprintln!("Failed to start server: {}", e),
    }

    // Wait for shutdown signal
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutdown signal received, exiting...");
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }
}
