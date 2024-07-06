use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}
