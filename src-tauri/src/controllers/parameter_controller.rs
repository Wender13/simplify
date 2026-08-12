use crate::{models::Parameter, repository::parameter_repository};

#[tauri::command]
pub async fn get_parameters() -> Result<Vec<Parameter>, String> {
    parameter_repository::get_parameters().await
}

