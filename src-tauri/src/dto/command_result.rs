use serde::Serialize;

#[derive(Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub message_key: String,
}
