pub mod commands;
mod error;
mod sqlite;
mod tray;
use std::sync::Mutex;

use tauri::{Manager, RunEvent};
use tauri_plugin_log::{Target, TargetKind};

pub struct AppState {
    pub is_quitting: Mutex<bool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                ])
                .build(),
        )
        .setup(|app| {
            app.manage(AppState {
                is_quitting: Mutex::new(false),
            });
            tray::init_tray(app)?;
            sqlite::set_db(app).map_err(|e| e.to_string())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_task,
            commands::list_categories,
            commands::list_tasks,
            commands::start_task,
            commands::stop_task,
            commands::finish_task,
            commands::toggle_task_done,
            commands::get_today_stats,
            commands::get_week_stats,
            commands::get_month_stats,
            commands::get_daily_focus,
            commands::upsert_daily_focus,
            commands::toggle_daily_focus,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                let state = app_handle.state::<AppState>();
                let is_quitting = *state.is_quitting.lock().unwrap();
                if !is_quitting {
                    api.prevent_exit();
                }
            }
        });
}
