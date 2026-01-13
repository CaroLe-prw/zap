use serde::Serialize;
use sqlx::{SqlitePool, prelude::FromRow};

use crate::error::ZapError;

#[derive(Debug, Serialize, FromRow)]
pub struct CategoryResponse {
    /// 主键
    id: u32,
    /// 分类名称
    name: String,
    /// 分类颜色
    color: String,
}

pub async fn list_categories_impl(pool: &SqlitePool) -> Result<Vec<CategoryResponse>, ZapError> {
    let categories_table = sqlx::query_as::<_, CategoryResponse>(
        r#"SELECT id, name, color, created_at FROM categories ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(categories_table)
}
