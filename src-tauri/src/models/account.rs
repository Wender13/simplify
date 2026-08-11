#[derive(sqlx::FromRow)]
pub struct Account {
    id_account: i64,
    id_user: i64,
    name: String,
    r#type: String,
    opening_balance: f64,
    currency: String,
}
