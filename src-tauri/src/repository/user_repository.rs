use crate::db;

pub enum CreateUserResult {
    Created,
    EmailTaken,
}

pub enum DeleteUserResult {
    Deleted,
    UserNotFound,
}

pub async fn delete_user(email: String) -> Result<DeleteUserResult, String> {
    let pool = db::pool();
    let result = sqlx::query("DELETE FROM user WHERE email = ?")
        .bind(email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(DeleteUserResult::UserNotFound)
    } else {
        Ok(DeleteUserResult::Deleted)
    }
}

pub async fn create_user(
    name: String,
    email: String,
    password: String,
) -> Result<CreateUserResult, String> {
    let pool = db::pool();
    let result = sqlx::query("INSERT INTO user (name, email, password) values (?,?,?)")
        .bind(name)
        .bind(email)
        .bind(password)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(CreateUserResult::Created),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Ok(CreateUserResult::EmailTaken)
        }
        Err(e) => Err(e.to_string()),
    }
}
