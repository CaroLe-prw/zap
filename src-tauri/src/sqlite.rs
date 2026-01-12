use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tauri::{App, Manager};

use crate::error::ZapError;

#[derive(Debug)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

async fn init_db(db_url: &str) -> Result<SqlitePool, ZapError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    let _ = sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await;

    let _ = sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub fn set_db(app: &mut App) -> Result<(), ZapError> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("zap.sqlite3");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        match init_db(&db_url).await {
            Ok(pool) => {
                handle.manage(Db::new(pool));
            }
            Err(e) => log::error!("init_db failed: {e}"),
        }
    });
    Ok(())
}
