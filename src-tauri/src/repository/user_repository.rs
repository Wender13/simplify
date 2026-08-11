use crate::db;
use crate::models::User;
pub enum CreateUserResult {
    Created,
    EmailTaken,
}

pub enum DeleteUserResult {
    Deleted,
    UserNotFound,
}

pub enum UpdateUserResult {
    Updated,
    UserNotFound,
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

pub async fn get_users() -> Result<Vec<User>, String> {
    let pool = db::pool();
    sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_user(email: String) -> Result<Option<User>, String> {
    let pool = db::pool();
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE email = ?")
        .bind(email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_user_complete(
    name: String,
    old_email: String,
    new_email: String,
    password: String,
) -> Result<UpdateUserResult, String> {
    let pool = db::pool();

    let result = sqlx::query("UPDATE user SET name = ?, password = ?, email= ? WHERE email = ?")
        .bind(name)
        .bind(password)
        .bind(new_email)
        .bind(old_email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(UpdateUserResult::UserNotFound)
    } else {
        Ok(UpdateUserResult::Updated)
    }
}

pub async fn update_user_name(name: String, email: String) -> Result<UpdateUserResult, String> {
    let pool = db::pool();

    let result = sqlx::query("UPDATE user SET name = ? WHERE email = ?")
        .bind(name)
        .bind(email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(UpdateUserResult::UserNotFound)
    } else {
        Ok(UpdateUserResult::Updated)
    }
}

pub async fn update_user_password(
    password: String,
    email: String,
) -> Result<UpdateUserResult, String> {
    let pool = db::pool();

    let result = sqlx::query("UPDATE user SET password = ? WHERE email = ?")
        .bind(password)
        .bind(email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(UpdateUserResult::UserNotFound)
    } else {
        Ok(UpdateUserResult::Updated)
    }
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
