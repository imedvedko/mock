use crate::model::Audit;
use crate::model::Result;
use crate::model::User;
use crate::repository::AuditRepository;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Audit")]
#[get("/audit")]
pub async fn get(mut repository: AuditRepository, _user: User) -> Result<Json<Vec<Audit>>> {
    let audit = repository.get().await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/tokens/<token>/audit")]
pub async fn token_get(
    mut repository: AuditRepository,
    _user: User,
    token: String,
) -> Result<Json<Vec<Audit>>> {
    let audit = repository.token_get(token).await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/mocks/<name>/audit")]
pub async fn mock_get(
    mut repository: AuditRepository,
    _user: User,
    name: String,
) -> Result<Json<Vec<Audit>>> {
    let audit = repository.mock_get(name).await?;

    Ok(Json(audit))
}
