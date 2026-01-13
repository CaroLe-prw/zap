pub mod task;
pub use crate::commands::task::{CreateTaskRequest, add_task_impl};
use crate::{error::ZapError, sqlite::Db};

#[tauri::command]
pub async fn add_task(
    db: tauri::State<'_, Db>,
    create_task: CreateTaskRequest,
) -> Result<(), ZapError> {
    add_task_impl(&db.pool, create_task).await
}
