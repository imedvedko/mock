use crate::model::Log;
use crate::model::Result;
use crate::model::User;
use crate::repository::LogRepository;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Log")]
#[get("/log")]
pub async fn get(mut repository: LogRepository, _user: User) -> Result<Json<Vec<Log>>> {
    let logs = repository.get().await?;

    Ok(Json(logs))
}

#[openapi(tag = "Log")]
#[get("/mocks/<name>/log")]
pub async fn mock_get(
    mut repository: LogRepository,
    _user: User,
    name: String,
) -> Result<Json<Vec<Log>>> {
    let logs = repository.mock_get(name).await?;

    Ok(Json(logs))
}
