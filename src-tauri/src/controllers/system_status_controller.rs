use crate::{models::SystemStatus, repository::system_status_repository};

#[tauri::command]
pub async fn get_system_status() -> Result<Vec<SystemStatus>, String> {
  system_status_repository::get_system_status().await
}