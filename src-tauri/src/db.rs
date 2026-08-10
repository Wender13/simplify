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
    sqlx::migrate!("./migrations/")
        .run(pool)
        .await
        .expect("Failed to run sql migration files!");

    Ok(())
}
