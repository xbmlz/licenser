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

static PRIVATE_KEY_PEM: &str = r#"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDCIJpLRXjHLiJc
xiGjjYvdw0LCO4AoiUDA7zuksGUd0PJBbqKvyeGezthbacpYrKtyhnHoCSNMmNAg
1yMQy33NGKcimviTqO9jHb0c1NfdbPwG9NGaJ3Rq8DTtjFvWDPGwIDQFZC8DiRbw
e3E+5FtfYhbeDySfUYtdxrfZvjI5xIAo9bmw6XVZQJOIy4+kP8zYvItzm5DtInfn
SYAhFm9E+204RDN4TFZMN6zcAi1H1q2yOoMY+9wza8z2w8CMWcq/jaEVwn32wrde
y1v2lBe5xZ9qsXMLVA2uqN9k5U1eMuJ3I0ac1zmioftFNjUcSE5FN/IBe7+7roxz
mg9S3vbzAgMBAAECggEAC08TMo+l727TTeyQ6NJhaoP8gcNSdPfLGOkW5EIy+vHT
mw2lVpUkmqWi/wFwRYp2JCJCVw5EpphkYdDPxf8gM7mApn9LPCSbIOycuiX28nsk
t3cGyJcx0W4mUp6j6Bnop6YkCtGBIMd0tRRL4QWiAjIPX9SXbyW00pdH/koID0tb
GiFujawubpmiy9BG0e65jkuVgHMD1/HBKtx9DqmgCcsDFjSPvT1CVFmy6ilzB02N
+DrGoUPDpVuKYtH8dQSwzovVs0YJqmJwY+bH1NNTZ0x354Qmg4Ht6wZmyi/uLFZj
MUOexVPU8n9yT0z/rny6ZgIo8Xet7ogSSasLQqo36QKBgQDfCcPAiTwAXlpRAVeR
LsB/81LTtVgX6NxZHR/cPnmoVgyKgRz7DFGdmgOhEoRF3SX0ZCqRwQqEhzsqHrqK
Y6t5lcOnql2COX7Cxyc10Rha0One4/RBgz9vooCJDKTnPIr4sGmw04W0EmvlAy3M
ba9mglnI5oG9j+fTp0qibL8aVQKBgQDe0Q1HKOSJ/fNbSh+IB15m5G+kQdhSL5uT
t9eD0GK4KcOnYy2akBohn1cbUvSYQMMWd067tS2LDlfyUPJMje5dXFLEBMXGLHid
8jp2T6Y8ADYMuBYr1fd/TUOaIm0vgNzecs5P0xZEOMHFnBfwDw8+MQIwyykZ3XBI
ePW7Y9YkJwKBgDsjWq0VdjxeyDHMWkybid0jRmXuIoKMcsiKKWV7h0R0NHUREP8b
4BQavzWZNEtV/PdVC9iDx+cl+DEN3sZM8S2W4T72tD6QQiUhKytg2sVRuYEpDh3E
0DAodU5hdOP/MJYKKKwDGeOKMuOROTaIKsbSbz4OqH37xyteozJ4BR/VAoGATuMO
42nL+DssBN8qaLvLJXytNieF0hs+5r7JE8ccnH1U4xePFtD8H3lNmsP1C06qg3K8
MmMD+96ZLpaQIqCBixZby0CxUOd/0NPo9OhgP5AHkts+Jkj79ltBmvmjVJU4HZ3i
A4sFsCO0HyWTqA984xTw5JuZMqoezdndjnnbYDMCgYEAzqf2iileyfXY02u6gTBT
IyOcpPcaCcyAYR0w/+U17YLzb5e7jKQtSkcnSMWVi1qkRIrocBmKyRTrY9OwfEnR
rJD0rILgnD0aMLvVN4IFM3nFkNXJ1EeYsaLfvsFa/C0zX9QMdhOyrgohfl1A+nc/
gQSqEKAArSPeobawKGLcdo4=
-----END PRIVATE KEY-----
"#;
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
