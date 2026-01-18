pub mod commands;
mod error;
mod sqlite;
use tauri_plugin_log::{Target, TargetKind};

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
