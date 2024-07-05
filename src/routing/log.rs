use crate::model::Log;
use crate::model::Result;
use crate::model::User;
use crate::repository::MockData;
use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

#[openapi(tag = "Log")]
#[get("/log")]
pub async fn get(db: Connection<MockData>, _user: User) -> Result<Json<Vec<Log>>> {
    let logs = crate::repository::log::get(db).await?;

    Ok(Json(logs))
}

#[openapi(tag = "Log")]
#[get("/mocks/<name>/log")]
pub async fn mock_get(db: Connection<MockData>, _user: User, name: &str) -> Result<Json<Vec<Log>>> {
    let logs = crate::repository::log::mock_get(db, name).await?;

    Ok(Json(logs))
}
