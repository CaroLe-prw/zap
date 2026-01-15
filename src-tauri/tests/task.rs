mod common;
use sqlx::SqlitePool;
use zap_lib::commands::{self, task::TaskStatus};

#[tokio::test]
async fn test_add_task_basic() {
    let pool: SqlitePool = common::setup_test_db()
        .await
        .expect("Failed to setup test database");

    let req = commands::CreateTaskRequestBuilder::default()
        .title("测试任务")
        .build()
        .unwrap();

    commands::add_task_impl(&pool, req)
        .await
        .expect("Failed to add task");
}

#[tokio::test]
async fn test_list_tasks() {
    let pool: SqlitePool = common::setup_test_db()
        .await
        .expect("Failed to setup test database");

    let req = commands::TaskQueryBuilder::default()
        .done(TaskStatus::Todo)
        .page_size(10u32)
        .page_index(1u32)
        .build()
        .unwrap();
    let task_table = commands::list_tasks_impl(&pool, req)
        .await
        .expect("Failed query  list tasks");

    println!("{:?}", task_table);
}
