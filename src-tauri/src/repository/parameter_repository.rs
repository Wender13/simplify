use crate::{db, models::Parameter};

pub enum UpdateParameterResult {
    ParameterUpdated,
    ParameterNotUpdated,
}

pub async fn get_parameters() -> Result<Vec<Parameter>, String> {
    let pool = db::pool();

    sqlx::query_as::<_, Parameter>("SELECT id_parameter, name, value FROM parameters")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_parameter(
    id_parameter: i64,
    value: String,
) -> Result<UpdateParameterResult, String> {
    let pool = db::pool();

    let result =
        sqlx::query("UPDATE parameters SET value = ? WHERE id_parameter = ? AND value != ?")
            .bind(&value)
            .bind(&id_parameter)
            .bind(&value)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Ok(UpdateParameterResult::ParameterNotUpdated)
    } else {
        Ok(UpdateParameterResult::ParameterUpdated)
    }
}
