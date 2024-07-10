use crate::model::{Conflict, Mock, Result, User};
use crate::repository::MockRepository;
use rocket::response::status::{Created, NoContent};
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
) -> Result<Conflict<Created<Json<Mock>>>> {
    let mock = mock.into_inner();
    let mock_name = mock.name.clone();
    let result = repository.create(user, mock).await;
    let responder = match result {
        Err(rocket::response::Debug(sqlx::Error::Database(error)))
            if error.is_unique_violation() =>
        {
            Conflict::error(mock_name)
        }
        _ => {
            let mock = result?;
            let location = rocket::uri!(get(&mock.name)).to_string();

            Conflict::ok(Created::new(location).body(Json(mock)))
        }
    };

    Ok(responder)
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
) -> Result<Conflict<Option<Json<Mock>>>> {
    let mock = mock.into_inner();
    let mock_name = mock.name.clone();
    let result = repository.update(user, name, mock).await;
    let responder = match result {
        Err(rocket::response::Debug(sqlx::Error::Database(error)))
            if error.is_unique_violation() =>
        {
            Conflict::error(mock_name)
        }
        _ => {
            let mock = result?.map(Json);
            Conflict::ok(mock)
        }
    };

    Ok(responder)
}

#[openapi(tag = "Mock")]
#[delete("/mocks/<name>")]
pub async fn delete(
    mut repository: MockRepository,
    user: User,
    name: String,
) -> Result<Option<NoContent>> {
    let response = repository.delete(user, name).await?.map(|_| NoContent);

    Ok(response)
}
