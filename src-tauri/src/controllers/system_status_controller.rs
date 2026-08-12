use crate::{dto::command_result::CommandResult, models::SystemStatus, repository::system_status_repository::{self, UpdateSystemStatusResult}};

#[tauri::command]
pub async fn get_system_status() -> Result<Vec<SystemStatus>, String> {
  system_status_repository::get_system_status().await
}

#[tauri::command]
pub async fn update_system_status(system_status: Vec<SystemStatus>) -> Result<CommandResult, String> {
    let mut list_fail = Vec::new();
    for s_s in system_status {
        let result =
            system_status_repository::update_system_status(s_s.id_system_status, s_s.value).await?;
        match result {
            UpdateSystemStatusResult::SystemStatusUpdated => {}
            UpdateSystemStatusResult::SystemStatusNotUpdated => {
                list_fail.push(s_s.name);
            }
        }
    }
    Ok(CommandResult {
        success: list_fail.is_empty(),
        message_key: if list_fail.is_empty() {
            "system_status.updated_sucessfully".into()
        } else {
            "system_status.failed_to_update".into()
        },
        list_fail,
    })
}