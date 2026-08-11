#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Parameter {
    pub id_parameter: i64,
    pub name: String,
    pub value: String,
}
