#[derive(sqlx::FromRow)]
pub struct License {
    pub id: i64,
    pub org_name: String,
    pub machine_id: String,
    pub license: String,
    pub max_users: u32,
    pub expires_at: String,
    pub created_at: String,
}
