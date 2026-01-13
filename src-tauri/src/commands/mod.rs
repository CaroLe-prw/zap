pub mod categories;
pub mod task;
pub use crate::commands::{
    categories::{CategoryResponse, list_categories_impl},
    task::{CreateTaskRequestBuilder, add_task_impl},
};
use crate::{commands::task::CreateTaskRequest, error::ZapError, sqlite::Db};

#[tauri::command]
pub async fn add_task(
    db: tauri::State<'_, Db>,
    create_task: CreateTaskRequest,
) -> Result<(), ZapError> {
    add_task_impl(&db.pool, create_task).await
}

#[tauri::command]
pub async fn list_categories(db: tauri::State<'_, Db>) -> Result<Vec<CategoryResponse>, ZapError> {
    list_categories_impl(&db.pool).await
}
