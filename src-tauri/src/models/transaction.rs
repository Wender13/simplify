use chrono::{DateTime, Utc};

pub struct Transaction {
  id_trasaction: i64,
  id_user: i64,
  id_category: i64,
  id_account: i64,
  value: f64,
  r#type: TransactionType,
  description: String,
  transaction_date: DateTime<Utc>,
}

enum TransactionType {
  Income,
  Expense,
}