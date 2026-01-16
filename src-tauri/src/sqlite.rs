use std::str::FromStr;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
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
    let options = SqliteConnectOptions::from_str(db_url)?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub fn set_db(app: &mut App) -> Result<(), ZapError> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let db_path = app_data_dir.join("zap.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    let handle = app.handle().clone();

    let pool = tauri::async_runtime::block_on(async move { init_db(&db_url).await })?;
    handle.manage(Db::new(pool));
    Ok(())
}
