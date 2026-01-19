use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::{
    QueryBuilder, Sqlite, SqlitePool, Transaction,
    prelude::{FromRow, Type},
};

use crate::{commands::types::PaginatedResponse, error::ZapError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Type, Default)]
#[repr(u8)]
pub enum TaskStatus {
    #[default]
    Todo = 0,
    Running = 1,
    Finished = 2,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tasks {
    /// 任务Id
    id: u32,
    /// 任务标题
    title: String,
    /// 任务运行状态
    done: TaskStatus,
    /// 分类id
    category_id: Option<u32>,
    ///预估用时
    estimate_seconds: Option<u32>,
    /// 任务备注
    notes: Option<String>,
    ///是否加入 Today Focus：0=否，1=是
    is_today_focus: Option<bool>,
    ///创建时间
    created_at: String,
    ///更新时间
    updated_at: String,
    /// 完成时间
    completed_at: String,
}

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
    #[builder(default)]
    is_today_focus: Option<bool>,
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
    done: Option<TaskStatus>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TaskResponse {
    /// 任务Id
    task_id: u32,
    /// 任务标题
    title: String,
    /// 任务状态
    done: TaskStatus,
    /// 分类id
    category_id: Option<u32>,
    /// 分类名称
    category_name: Option<String>,
    /// 分类颜色
    color: Option<String>,
    /// 实际运行总时长（秒）
    total_duration_seconds: i64,
    /// 今日运行时长（秒）
    today_duration_seconds: i64,
    /// 完成时间
    completed_at: Option<String>,
    /// 当前会话时长(秒)
    session_seconds: i64,
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
            .fetch_one(tx.as_mut())
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
    .execute(tx.as_mut())
    .await?;

    let task_id = res.last_insert_rowid();

    if req.start_on_create == Some(true) {
        sqlx::query("INSERT INTO time_entries (task_id, started_at, note) VALUES (?, ?, ?)")
            .bind(task_id)
            .bind(&now)
            .bind(format!("{} #1", req.title))
            .execute(tx.as_mut())
            .await?;

        update_task_status(&mut tx, task_id as u32, TaskStatus::Running).await?;
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
        "SELECT t.id AS task_id, t.title, t.done, t.category_id, c.name AS category_name, c.color, \
         COALESCE((SELECT SUM(duration_seconds) FROM time_entries WHERE task_id = t.id), 0) AS total_duration_seconds, \
         COALESCE((SELECT SUM(duration_seconds) FROM time_entries WHERE task_id = t.id AND date(started_at) = date('now')), 0) AS today_duration_seconds, \
         t.completed_at, \
         CASE WHEN t.done = 1 THEN CAST((strftime('%s', 'now') - strftime('%s', te.started_at)) AS INTEGER) ELSE 0 END AS session_seconds \
         FROM tasks t \
         LEFT JOIN categories c ON t.category_id = c.id \
         LEFT JOIN time_entries te ON t.id = te.task_id AND te.ended_at IS NULL \
         WHERE 1=1",
    );
    apply_filters(&mut qb, &req);
    qb.push(" ORDER BY t.done ASC,t.created_at DESC");

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

pub async fn start_task_impl(pool: &SqlitePool, task_id: u32) -> Result<(), ZapError> {
    let task = get_task_by_id(pool, task_id).await?;
    if task.done != TaskStatus::Todo {
        return Err(ZapError::TaskAlreadyStarted(task_id));
    }

    let mut tx = pool.begin().await?;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM time_entries WHERE task_id = ?")
        .bind(task_id)
        .fetch_one(tx.as_mut())
        .await?;

    let next_session_num = count + 1;
    let note = format!("{} #{}", task.title, next_session_num);

    update_task_status(&mut tx, task_id, TaskStatus::Running).await?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    sqlx::query("INSERT INTO time_entries (task_id, started_at, note) VALUES (?, ?, ?)")
        .bind(task_id)
        .bind(now)
        .bind(note)
        .execute(tx.as_mut())
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn stop_task_impl(pool: &SqlitePool, task_id: u32) -> Result<(), ZapError> {
    let task = get_task_by_id(pool, task_id).await?;
    if task.done != TaskStatus::Running {
        return Err(ZapError::TaskNotStarted(task_id));
    }
    let mut tx = pool.begin().await?;

    update_task_status(&mut tx, task_id, TaskStatus::Todo).await?;
    update_time_entries(&mut tx, task_id).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn finish_task_impl(pool: &SqlitePool, task_id: u32) -> Result<(), ZapError> {
    let task = get_task_by_id(pool, task_id).await?;

    let mut tx = pool.begin().await?;

    if task.done == TaskStatus::Running {
        update_task_status(&mut tx, task_id, TaskStatus::Todo).await?;
        update_time_entries(&mut tx, task_id).await?;
    }

    update_task_status(&mut tx, task_id, TaskStatus::Finished).await?;

    sqlx::query("UPDATE tasks SET completed_at = datetime('now')  WHERE id = ?")
        .bind(task_id)
        .execute(tx.as_mut())
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn toggle_task_done_impl(pool: &SqlitePool, task_id: u32) -> Result<(), ZapError> {
    let task = get_task_by_id(pool, task_id).await?;
    if task.done != TaskStatus::Finished {
        return Err(ZapError::TaskNotDone(task_id));
    }
    let mut tx = pool.begin().await?;
    update_task_status(&mut tx, task_id, TaskStatus::Todo).await?;
    tx.commit().await?;
    Ok(())
}

async fn update_task_status(
    tx: &mut Transaction<'_, Sqlite>,
    task_id: u32,
    done: TaskStatus,
) -> Result<(), ZapError> {
    sqlx::query("UPDATE tasks SET done = ? WHERE id = ?")
        .bind(done)
        .bind(task_id)
        .execute(tx.as_mut())
        .await?;

    Ok(())
}

async fn update_time_entries(
    tx: &mut Transaction<'_, Sqlite>,
    task_id: u32,
) -> Result<(), ZapError> {
    sqlx::query(
        r#"
        UPDATE time_entries
        SET
            ended_at = datetime('now'),
            duration_seconds = CAST((strftime('%s', 'now') - strftime('%s', started_at)) AS INTEGER)
        WHERE task_id = ? AND ended_at IS NULL
        "#,
    )
    .bind(task_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

async fn get_task_by_id(pool: &SqlitePool, task_id: u32) -> Result<Tasks, ZapError> {
    sqlx::query_as::<_, Tasks>(
        "SELECT id , title, done, category_id, estimate_seconds, notes, is_today_focus, created_at, updated_at, completed_at FROM tasks WHERE id = ?",
    ).bind(task_id).fetch_optional(pool).await?.ok_or(ZapError::TaskNotFound(task_id))
}

fn apply_filters<'a>(qb: &mut QueryBuilder<'a, Sqlite>, req: &'a TaskQuery) {
    if let Some(ref name) = req.task_name {
        qb.push(" AND t.title LIKE ");
        qb.push_bind(format!("%{}%", name));
    }
    if let Some(done) = req.done {
        qb.push(" AND t.done = ");
        qb.push_bind(done);
    } else {
        qb.push(" AND t.done IN (0, 1)");
    }
}
