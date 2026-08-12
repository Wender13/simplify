#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub id_parameter: i64,
    pub name: String,
    pub value: String,
}
