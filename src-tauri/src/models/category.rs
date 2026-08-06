pub struct Category {
    id_category: i64,
    id_user: i64,
    name: String,
    r#type: CategoryType,
    color: String,
    icon: String,
}

enum CategoryType {
  Income,
  Expense,
}