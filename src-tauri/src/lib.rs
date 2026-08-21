mod commands;
mod db;
mod models;

use commands::{
    create_account, create_operation, delete_account, delete_operation, list_accounts,
    list_operations, update_account, update_operation,
};
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = app.path().app_data_dir()?.join("money-cockpit.db");
            let conn = db::init(db_path)?;
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_account,
            update_account,
            delete_account,
            list_accounts,
            create_operation,
            update_operation,
            delete_operation,
            list_operations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
