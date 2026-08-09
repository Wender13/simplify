use sqlx::SqlitePool;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn set_app_handle(handle: AppHandle) {
    APP_HANDLE.set(handle).expect("App handle already started!");
}

pub fn pool() -> SqlitePool {
    APP_HANDLE
        .get()
        .expect("App handle not started yet!")
        .state::<SqlitePool>()
        .inner()
        .clone()
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("../migrations/001_user.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/002_category.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/003_account.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/004_transaction.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/005_financial_target.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/006_indexes.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../migrations/007_trigger.sql"))
        .execute(pool)
        .await?;

    Ok(())
}
