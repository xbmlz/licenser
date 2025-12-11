use askama::Template;
use axum::{
    Form, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get},
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{models::License, routes::HtmlTemplate, session::Admin};

static PRIVATE_KEY_PEM: &str = include_str!("../../private.pem");

#[derive(Template)]
#[template(path = "licenses.html")]
pub struct LicensesPage {
    licenses: Vec<License>,
}

#[derive(Template)]
#[template(path = "licenses_table.html")]
pub struct LicensesTable {
    licenses: Vec<License>,
}

#[derive(Deserialize)]
pub struct NewLicense {
    org_name: String,
    max_users: u32,
    machine_id: String,
    expires_at: String,
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(list_page).post(create_license))
        .route("/{id}", delete(delete_license))
}

async fn list_page(Admin: Admin, State(pool): State<SqlitePool>) -> impl IntoResponse {
    let list = sqlx::query_as::<_, License>("SELECT * FROM licenses ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap();
    let tpl = LicensesPage { licenses: list };
    HtmlTemplate(tpl)
}

async fn create_license(
    Admin: Admin,
    State(pool): State<SqlitePool>,
    Form(form): Form<NewLicense>,
) -> impl IntoResponse {
    let payload = core::license::LicensePayload {
        org_name: form.org_name,
        max_users: form.max_users,
        machine_id: form.machine_id,
        expires_at: form.expires_at,
    };
    let license = core::license::generate_license(&payload, PRIVATE_KEY_PEM).unwrap();
    sqlx::query("INSERT INTO licenses (org_name, max_users, machine_id, license, expires_at) VALUES (?, ?, ?, ?, ?)")
    .bind(payload.org_name)
    .bind(payload.max_users)
    .bind(payload.machine_id)
    .bind(license)
    .bind(payload.expires_at)
    .execute(&pool)
    .await
    .unwrap();
    let list = sqlx::query_as::<_, License>("SELECT * FROM licenses ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap();
    let tpl = LicensesTable { licenses: list };
    HtmlTemplate(tpl)
}

async fn delete_license(
    Admin: Admin,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query("DELETE FROM licenses WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    HtmlTemplate(LicensesTable {
        licenses: Vec::new(),
    })
}
