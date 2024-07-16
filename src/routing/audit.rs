use crate::model::{Audit, PageRequest, Result, User};
use crate::repository::AuditRepository;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Audit")]
#[get("/audit?<page_request..>")]
pub async fn get(
    mut repository: AuditRepository,
    _user: User,
    page_request: PageRequest,
) -> Result<Json<Vec<Audit>>> {
    let audit = repository.get(page_request).await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/tokens/<token>/audit?<page_request..>")]
pub async fn token_get(
    mut repository: AuditRepository,
    _user: User,
    token: String,
    page_request: PageRequest,
) -> Result<Json<Vec<Audit>>> {
    let audit = repository.token_get(token, page_request).await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/mocks/<name>/audit?<page_request..>")]
pub async fn mock_get(
    mut repository: AuditRepository,
    _user: User,
    name: String,
    page_request: PageRequest,
) -> Result<Json<Vec<Audit>>> {
    let audit = repository.mock_get(name, page_request).await?;

    Ok(Json(audit))
}
