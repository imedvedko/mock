use crate::model::Mock;
use crate::model::Result;
use crate::model::User;
use crate::repository::MockRepository;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

#[openapi(tag = "Mock")]
#[get("/mocks")]
pub async fn list(mut repository: MockRepository, _user: User) -> Result<Json<Vec<String>>> {
    let mocks = repository.list().await?;

    Ok(Json(mocks))
}

#[openapi(tag = "Mock")]
#[post("/mocks", data = "<mock>")]
pub async fn create(
    mut repository: MockRepository,
    user: User,
    mock: Json<Mock>,
) -> Result<Created<Json<Mock>>> {
    let mock = repository.create(user, mock.into_inner()).await?;
    let location = rocket::uri!(get(&mock.name)).to_string();

    Ok(Created::new(location).body(Json(mock)))
}

#[openapi(tag = "Mock")]
#[get("/mocks/<name>")]
pub async fn get(
    mut repository: MockRepository,
    _user: User,
    name: String,
) -> Result<Option<Json<Mock>>> {
    let response = repository.get(name).await?.map(Json);

    Ok(response)
}

#[openapi(tag = "Mock")]
#[put("/mocks/<name>", data = "<mock>")]
pub async fn update(
    mut repository: MockRepository,
    user: User,
    name: String,
    mock: Json<Mock>,
) -> Result<Option<Json<Mock>>> {
    let mock = mock.into_inner();
    let mock = repository.update(user, name, mock).await?;
    let response = mock.map(Json);

    Ok(response)
}

#[openapi(tag = "Mock")]
#[delete("/mocks/<name>")]
pub async fn delete(
    mut repository: MockRepository,
    user: User,
    name: String,
) -> Result<Option<()>> {
    repository.delete(user, name).await
}
