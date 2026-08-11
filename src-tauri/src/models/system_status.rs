#[derive(sqlx::FromRow, serde::Serialize)]
pub struct SystemStatus {
    pub id_system_status: i64,
    pub name: String,
    pub value: String,
}
