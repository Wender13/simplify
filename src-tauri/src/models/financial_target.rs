use chrono::{DateTime, Utc};

pub struct FinancialTarget {
  id_target: i64,
  id_user: i64,
  id_category: i64,
  name: String,
  limit_value: f64,
  current_value: f64,
  start_date: DateTime<Utc>,
  end_date: DateTime<Utc>,
  active: bool
}