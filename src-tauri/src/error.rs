use sqlx::migrate::MigrateError;
use thiserror::Error;

/// ZAP 应用错误类型
#[derive(Debug, Error)]
pub enum ZapError {
    /// Tauri 错误（路径操作等）
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    /// 数据库操作失败（SQL 执行错误、连接失败等）
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    /// 数据库迁移失败
    #[error("Migration error: {0}")]
    Migration(#[from] MigrateError),
    /// 任务不存在（根据 ID 查询未命中）
    #[error("Task not found: id={0}")]
    TaskNotFound(u32),
    /// 任务已开始
    #[error("Task already started: id={0}")]
    TaskAlreadyStarted(u32),
    /// 任务未开始
    #[error("Task not started: id={0}")]
    TaskNotStarted(u32),
    /// 分类不存在（根据 ID 查询未命中）
    #[error("Category not found: id={0}")]
    CategoryNotFound(u32),
    /// 时间条目不存在（根据 ID 查询未命中）
    #[error("Time entry not found: id={0}")]
    TimeEntryNotFound(u32),
    /// 任务数据校验失败（标题为空、预估时间负数等）
    #[error("Invalid task data: {0}")]
    InvalidTaskData(&'static str),
    /// 分类数据校验失败（名称为空等）
    #[error("Invalid category data: {0}")]
    InvalidCategoryData(&'static str),
    /// 无法删除分类（该分类下存在关联任务）
    #[error("Category has associated tasks")]
    CategoryHasTasks,
    /// 文件/IO 操作失败
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for ZapError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
