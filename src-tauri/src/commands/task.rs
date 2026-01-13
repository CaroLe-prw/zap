use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::ZapError;

#[derive(Debug, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct CreateTaskRequest {
    /// 文本标题
    title: String,
    /// 分类id
    #[builder(default)]
    category_id: Option<u32>,
    /// 预估用时
    #[builder(default)]
    estimate_seconds: Option<u32>,
    /// 备注
    #[builder(default)]
    notes: Option<String>,
    /// 是否加入Today Focus
    #[builder(default = "false")]
    is_today_focus: bool,
    /// 是否立即开始并且开始计时
    #[builder(default)]
    start_on_create: Option<bool>,
}

pub async fn add_task_impl(pool: &SqlitePool, req: CreateTaskRequest) -> Result<(), ZapError> {
    if req.title.is_empty() {
        return Err(ZapError::InvalidTaskData("title cannot be empty"));
    }

    let mut tx = pool.begin().await?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 验证 category 存在
    if let Some(category_id) = req.category_id {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE id = ?")
            .bind(category_id)
            .fetch_one(&mut *tx)
            .await?;

        if count == 0 {
            return Err(ZapError::InvalidCategoryData("category id not found"));
        }
    }

    // 插入任务
    let res = sqlx::query(
        "INSERT INTO tasks (title, done, category_id, estimate_seconds, notes, is_today_focus)
          VALUES (?, 0, ?, ?, ?, ?)",
    )
    .bind(&req.title)
    .bind(req.category_id)
    .bind(req.estimate_seconds)
    .bind(&req.notes)
    .bind(req.is_today_focus)
    .execute(&mut *tx)
    .await?;

    let task_id = res.last_insert_rowid();

    if req.start_on_create == Some(true) {
        sqlx::query("INSERT INTO time_entries (task_id, started_at, note) VALUES (?, ?, ?)")
            .bind(task_id)
            .bind(&now)
            .bind(format!("{} #1", req.title))
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}
