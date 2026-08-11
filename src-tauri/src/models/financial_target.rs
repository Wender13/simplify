use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct FinancialTarget {
    id_target: i64,
    id_user: i64,
    id_category: i64,
    name: String,
    limit_value: f64,
    current_value: f64,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    active: bool,
}
