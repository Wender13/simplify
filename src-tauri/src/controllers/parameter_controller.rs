use crate::{
    dto::command_result::CommandResult,
    models::Parameter,
    repository::parameter_repository::{self, UpdateParameterResult},
};

#[tauri::command]
pub async fn get_parameters() -> Result<Vec<Parameter>, String> {
    parameter_repository::get_parameters().await
}

#[tauri::command]
pub async fn update_parameters(parameters: Vec<Parameter>) -> Result<CommandResult, String> {
    let mut list_fail = Vec::new();
    for parameter in parameters {
        let result =
            parameter_repository::update_parameter(parameter.id_parameter, parameter.value).await?;
        match result {
            UpdateParameterResult::ParameterUpdated => {}
            UpdateParameterResult::ParameterNotUpdated => {
                list_fail.push(parameter.name);
            }
        }
    }
    Ok(CommandResult {
        success: list_fail.is_empty(),
        message_key: if list_fail.is_empty() {
            "parameters.updated_sucessfully".into()
        } else {
            "parameters.failed_to_update".into()
        },
        list_fail,
    })
}
