use crate::model::Audit;
use crate::model::Result;
use crate::model::User;
use crate::repository::MockData;
use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

#[openapi(tag = "Audit")]
#[get("/audit")]
pub async fn get(db: Connection<MockData>, _user: User) -> Result<Json<Vec<Audit>>> {
    let audit = crate::repository::audit::get(db).await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/tokens/<token>/audit")]
pub async fn token_get(
    db: Connection<MockData>,
    _user: User,
    token: &str,
) -> Result<Json<Vec<Audit>>> {
    let audit = crate::repository::audit::token_get(db, token).await?;

    Ok(Json(audit))
}

#[openapi(tag = "Audit")]
#[get("/mocks/<name>/audit")]
pub async fn mock_get(
    db: Connection<MockData>,
    _user: User,
    name: &str,
) -> Result<Json<Vec<Audit>>> {
    let audit = crate::repository::audit::mock_get(db, name).await?;

    Ok(Json(audit))
}
