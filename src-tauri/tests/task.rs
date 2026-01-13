mod common;
use sqlx::SqlitePool;
use zap_lib::commands;

#[tokio::test]
async fn test_list_categories() {
    let pool: SqlitePool = common::setup_test_db()
        .await
        .expect("Failed to setup test database");

    let categories_table = commands::list_categories_impl(&pool)
        .await
        .expect("查询数据失败");

    println!("{:?}", categories_table);
}
