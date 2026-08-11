use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id_user: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}
