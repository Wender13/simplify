use sqlx::SqlitePool;
use tauri::Manager;

mod db;
mod commands;

pub type DbPool = SqlitePool;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                let db_path = app_handle
                    .path()
                    .app_data_dir()
                    .expect("erro ao obter diretório de dados")
                    .join("simplify.db");

                let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

                let pool = SqlitePool::connect(&db_url)
                    .await
                    .expect("erro ao conectar com o banco de dados");

                db::run_migrations(&pool)
                    .await
                    .expect("erro ao executar migrations");

                app_handle.manage(pool);
            });

            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
