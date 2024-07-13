use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
