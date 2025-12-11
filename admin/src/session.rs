use axum::{extract::FromRequestParts, http::request::Parts, response::Redirect};
use axum_extra::extract::CookieJar;

pub struct Admin;

impl<S> FromRequestParts<S> for Admin
where
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        if jar.get("admin").is_some() {
            Ok(Admin)
        } else {
            Err(Redirect::to("/login"))
        }
    }
}
