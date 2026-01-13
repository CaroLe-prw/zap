mod common;
use sqlx::SqlitePool;
use zap_lib::commands;

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
