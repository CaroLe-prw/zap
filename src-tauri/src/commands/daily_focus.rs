use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::ZapError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DailyFocus {
    pub id: i64,
    pub focus_date: String,
    pub content: String,
    pub is_done: i32,
    pub position: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertDailyFocusRequest {
    pub date: String,
    pub content: String,
}

pub async fn get_daily_focus_impl(
    pool: &SqlitePool,
    date: String,
) -> Result<Option<DailyFocus>, ZapError> {
    let focus = sqlx::query_as::<_, DailyFocus>(
        "SELECT id, focus_date, content, is_done, position, created_at, updated_at
         FROM daily_focus WHERE focus_date = ? LIMIT 1",
    )
    .bind(date)
    .fetch_optional(pool)
    .await?;

    Ok(focus)
}

pub async fn upsert_daily_focus_impl(
    pool: &SqlitePool,
    req: UpsertDailyFocusRequest,
) -> Result<DailyFocus, ZapError> {
    let existing = get_daily_focus_impl(pool, req.date.clone()).await?;

    if let Some(focus) = existing {
        sqlx::query(
            "UPDATE daily_focus SET content = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(&req.content)
        .bind(focus.id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query("INSERT INTO daily_focus (focus_date, content) VALUES (?, ?)")
            .bind(&req.date)
            .bind(&req.content)
            .execute(pool)
            .await?;
    }

    get_daily_focus_impl(pool, req.date)
        .await?
        .ok_or(ZapError::InvalidTaskData(
            "Failed to retrieve upserted focus",
        ))
}

pub async fn toggle_daily_focus_impl(pool: &SqlitePool, id: i64) -> Result<DailyFocus, ZapError> {
    sqlx::query(
        "UPDATE daily_focus SET is_done = 1 - is_done, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;

    sqlx::query_as::<_, DailyFocus>(
        "SELECT id, focus_date, content, is_done, position, created_at, updated_at
         FROM daily_focus WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ZapError::TaskNotFound(id as u32))
}
