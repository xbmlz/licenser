mod db;
mod models;
mod routes;
mod session;

use db::init_db;

use axum::Router;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let db = init_db().await;
    let app = Router::new()
        .merge(routes::auth::routes())
        .merge(routes::licenses::routes())
        .with_state(db);

    let addr = format!("0.0.0.0:{}", 3000);
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
