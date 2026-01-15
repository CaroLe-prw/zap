use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqlitePool, prelude::FromRow};

use crate::{commands::types::PaginatedResponse, error::ZapError};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct CreateTaskRequest {
    /// 文本标题
    title: String,
    /// 分类id
    #[builder(default)]
    category_id: Option<u32>,
    /// 预估用时（秒）
    #[builder(default)]
    estimate_seconds: Option<i64>,
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

#[derive(Debug, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct TaskQuery {
    /// 当前页码
    #[builder(default = "1")]
    page_index: u32,
    /// 每页数量
    #[builder(default = "20")]
    page_size: u32,
    /// 按任务名称模糊搜索
    #[builder(default)]
    task_name: Option<String>,
    /// 按任务状态分类
    #[builder(default)]
    done: Option<bool>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TaskResponse {
    /// 任务Id
    task_id: u32,
    /// 任务标题
    title: String,
    /// 任务状态
    done: bool,
    /// 分类id
    category_id: Option<u32>,
    /// 分类名称
    category_name: Option<String>,
    /// 分类颜色
    color: Option<String>,
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

pub async fn list_tasks_impl(
    pool: &SqlitePool,
    req: TaskQuery,
) -> Result<PaginatedResponse<TaskResponse>, ZapError> {
    let mut count_qb = QueryBuilder::<Sqlite>::new("SELECT COUNT(*) FROM tasks t WHERE 1=1");
    apply_filters(&mut count_qb, &req);
    let total: u32 = count_qb.build_query_as::<(i64,)>().fetch_one(pool).await?.0 as u32;

    if total == 0 {
        return Ok(PaginatedResponse::empty(req.page_index, req.page_size));
    }

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT t.id AS task_id, t.title, t.done, t.category_id, c.name AS category_name, c.color \
         FROM tasks t \
         LEFT JOIN categories c ON t.category_id = c.id \
         WHERE 1=1",
    );
    apply_filters(&mut qb, &req);
    qb.push(" ORDER BY t.id DESC");

    let offset = (req.page_index.saturating_sub(1)) * req.page_size;
    qb.push(" LIMIT ");
    qb.push_bind(req.page_size as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);

    let tasks = qb.build_query_as::<TaskResponse>().fetch_all(pool).await?;

    Ok(PaginatedResponse::new(
        tasks,
        total,
        req.page_index,
        req.page_size,
    ))
}

fn apply_filters<'a>(qb: &mut QueryBuilder<'a, Sqlite>, req: &'a TaskQuery) {
    if let Some(ref name) = req.task_name {
        qb.push(" AND t.title LIKE ");
        qb.push_bind(format!("%{}%", name));
    }
    if let Some(done) = req.done {
        qb.push(" AND t.done = ");
        qb.push_bind(done);
    }
}
