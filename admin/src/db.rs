use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let db = SqlitePool::connect("sqlite://app.db?mode=rwc")
        .await
        .expect("connect SQLite failed");

    // 启动时自动建表
    sqlx::query(include_str!("../migrations/init.sql"))
        .execute(&db)
        .await
        .expect("create table failed");
    db
}