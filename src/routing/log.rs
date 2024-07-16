use crate::model::{Log, PageRequest, Result, User};
use crate::repository::LogRepository;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Log")]
#[get("/log?<page_request..>")]
pub async fn get(
    mut repository: LogRepository,
    _user: User,
    page_request: PageRequest,
) -> Result<Json<Vec<Log>>> {
    let logs = repository.get(page_request).await?;

    Ok(Json(logs))
}

#[openapi(tag = "Log")]
#[get("/mocks/<name>/log?<page_request..>")]
pub async fn mock_get(
    mut repository: LogRepository,
    _user: User,
    name: String,
    page_request: PageRequest,
) -> Result<Json<Vec<Log>>> {
    let logs = repository.mock_get(name, page_request).await?;

    Ok(Json(logs))
}
