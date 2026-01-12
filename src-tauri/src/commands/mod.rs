use crate::{commands::task::CreateTaskRequest, error::ZapError, sqlite::Db};

pub mod task;

#[tauri::command]
pub async fn add_task(
    db: tauri::State<'_, Db>,
    create_task: CreateTaskRequest,
) -> Result<(), ZapError> {
    Ok(())
}
