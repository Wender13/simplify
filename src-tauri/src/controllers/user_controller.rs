use crate::dto::command_result::CommandResult;
use crate::repository::user_repository::{self, CreateUserResult, DeleteUserResult};

pub async fn create_user(
    name: String,
    email: String,
    password: String,
) -> Result<CommandResult, String> {
    let name = name;
    let email: String = email.to_lowercase().trim().into();
    let password = password.trim();

    if name.is_empty() || email.is_empty() || password.is_empty() {
        return Ok(CommandResult {
            success: false,
            message_key: "user.emptyUserData".into(),
        });
    }

    match user_repository::create_user(name, email.into(), password.into()).await? {
        CreateUserResult::Created => Ok(CommandResult {
            success: true,
            message_key: "user.userCreated".into(),
        }),
        CreateUserResult::EmailTaken => Ok(CommandResult {
            success: false,
            message_key: "user.emailTaken".into(),
        }),
    }
}

pub async fn delete_user(email: String) -> Result<CommandResult, String> {
    let email = email.to_lowercase();

    if email.is_empty() {
        return Ok(CommandResult {
            success: false,
            message_key: "user.emptyUserData".into(),
        });
    }

    match user_repository::delete_user(email).await? {
        DeleteUserResult::Deleted => Ok(CommandResult {
            success: true,
            message_key: "user.userDeleted".into(),
        }),
        DeleteUserResult::UserNotFound => Ok(CommandResult {
            success: false,
            message_key: "user.userNotFound".into(),
        }),
    }
}
