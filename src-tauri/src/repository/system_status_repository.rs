use crate::{db, models::SystemStatus};

pub enum UpdateSystemStatusResult {
    SystemStatusUpdated,
    SystemStatusNotUpdated,
}

pub async fn get_system_status() -> Result<String, String> {
    let pool = db::pool();

    let system_status = sqlx::query_as::<_, SystemStatus>(
        "SELECT id_system_status, name, value FROM system_status",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string());
    let system_status_json = serde_json::to_string(&system_status).map_err(|e| e.to_string())?;

    Ok(system_status_json)
}

pub async fn update_system_status(
    id_system_status: i64,
    value: String,
) -> Result<UpdateSystemStatusResult, String> {
    let pool = db::pool();

    let result =
        sqlx::query("UPDATE system_status SET value = ? WHERE id_system_status = ? AND value != ?")
            .bind(&value)
            .bind(&id_system_status)
            .bind(&value)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(UpdateSystemStatusResult::SystemStatusNotUpdated)
    } else {
        Ok(UpdateSystemStatusResult::SystemStatusUpdated)
    }
}
