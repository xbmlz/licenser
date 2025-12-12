use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let db = SqlitePool::connect("sqlite://license.db?mode=rwc")
        .await
        .expect("connect SQLite failed");

    sqlx::query(include_str!("../migrations/init.sql"))
        .execute(&db)
        .await
        .expect("create table failed");
    db
}
