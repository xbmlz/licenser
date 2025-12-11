use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::routes::HtmlTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub fn routes() -> Router<SqlitePool> {
    Router::new().route("/login", get(login_page).post(login_submit))
        .route("/logout", post(logout))
}

async fn login_page() -> impl IntoResponse {
    let template = LoginTemplate { error: None };
    HtmlTemplate(template)
}

async fn logout(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove(Cookie::new("admin", ""));
    (jar, Redirect::to("/login")).into_response()
}

async fn login_submit(
    State(_pool): State<SqlitePool>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let valid_user = "admin";
    let valid_password = "admin@123.";

    if form.username == valid_user && form.password == valid_password {
        let mut cookie = Cookie::new("admin", uuid::Uuid::new_v4().to_string());
        // max age 1 day
        cookie.set_max_age(time::Duration::days(1));
        let jar = jar.add(cookie);
        return (jar, Redirect::to("/")).into_response();
    }

    let template = LoginTemplate {
        error: Some("用户名或密码错误".to_string()),
    };
    HtmlTemplate(template).into_response()
}
