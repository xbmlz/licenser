use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    // mkdir data dir if not exist
    std::fs::create_dir_all("data").unwrap();
    let db = SqlitePool::connect("sqlite://data/license.db?mode=rwc")
        .await
        .expect("connect SQLite failed");

    sqlx::query(include_str!("../migrations/init.sql"))
        .execute(&db)
        .await
        .expect("create table failed");
    db
}
