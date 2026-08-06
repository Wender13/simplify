use chrono::{DateTime, Utc};

pub struct User {
  id: i64,
  name: String,
  email: String,
  password_hash: String,
  created_at: DateTime<Utc>,
  last_login: DateTime<Utc>,
}