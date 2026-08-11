use std::{str::FromStr, time::Duration};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tauri::Manager;

pub mod repository;
pub mod controllers;
pub mod models;
pub mod db;
pub mod dto;

pub type DbPool = SqlitePool;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let db_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get database directory!")
                .join("simplify");

            std::fs::create_dir_all(&db_dir).expect("Failed to create database path!");

            let db_path = db_dir.join("simplify.db");

            let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

            let connect_options = SqliteConnectOptions::from_str(&db_url)
                .expect("Failed to parse db url!")
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                .foreign_keys(true);

            let pool_options = SqlitePoolOptions::new()
                .max_connections(5)
                .min_connections(1)
                .acquire_timeout(Duration::from_secs(5))
                .idle_timeout(Duration::from_secs(300));

            let pool = tauri::async_runtime::block_on(async move {
                let pool = pool_options
                    .connect_with(connect_options)
                    .await
                    .expect("Failed to connect with database!");

                db::run_migrations(&pool)
                    .await
                    .expect("Failed to run migrations!");

                pool
            });

            app_handle.manage(pool);
            db::set_app_handle(app_handle);

            tauri::async_runtime::block_on(test_new_user());
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn test_new_user() {
//     println!("---Before testing---");

//     let name = "User name teste".into();
//     let email = "test@domain.dev".into();
//     let password = "passwordTest".into();

//     let result = controllers::user_controller::create_user(name, email, password).await;

//     println!("---After testing---");
//     println!("---Result---");

//     match result {
//         Ok(command_result) => {
//             println!("Sucess: {}", command_result.success);
//             println!("Message key: {}", command_result.message_key);
//         }
//         Err(e) => println!("Error: {}", e.to_string()),
//     }
// }

async fn test_new_user() {
    println!("---Before testing---");

    let email = "test@domain.dev".into();

    let result = controllers::user_controller::delete_user(email).await;

    println!("---After testing---");
    println!("---Result---");

    match result {
        Ok(command_result) => {
            println!("Sucess: {}", command_result.success);
            println!("Message key: {}", command_result.message_key);
        }
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
