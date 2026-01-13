mod common;
use sqlx::SqlitePool;
use zap_lib::commands;

#[tokio::test]
async fn test_add_task_basic() {
    let pool: SqlitePool = common::setup_test_db()
        .await
        .expect("Failed to setup test database");

    let req = commands::CreateTaskRequest {
        title: "Test Task".into(),
        category_id: None,
        estimate_seconds: None,
        notes: None,
        is_today_focus: false,
        start_on_create: None,
    };

    commands::add_task_impl(&pool, req)
        .await
        .expect("Failed to add task");
}
