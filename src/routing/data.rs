use crate::model::{BigString, Conflict, Mock, Result, User};
use crate::repository::MockRepository;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, put};
use rocket_okapi::openapi;

#[openapi(tag = "Data")]
#[post("/mocks/<name>/data", data = "<response>")]
pub async fn create(
    mut repository: MockRepository,
    user: User,
    name: String,
    response: BigString,
) -> Result<Conflict<Created<Json<Mock>>>> {
    let mock = Mock {
        name: name.clone(),
        response: response.value,
    };
    let result = repository.create(user, mock).await;
    let responder = match result {
        Err(rocket::response::Debug(sqlx::Error::Database(error)))
            if error.is_unique_violation() =>
        {
            Conflict::error(name)
        }
        _ => {
            let mock = result?;
            let location = rocket::uri!(crate::routing::mock::get(&mock.name)).to_string();

            Conflict::ok(Created::new(location).body(Json(mock)))
        }
    };

    Ok(responder)
}

#[openapi(tag = "Data")]
#[put("/mocks/<name>/data", data = "<response>")]
pub async fn update(
    mut repository: MockRepository,
    user: User,
    name: String,
    response: BigString,
) -> Result<Option<Json<Mock>>> {
    let mock = Mock {
        name: name.clone(),
        response: response.value,
    };
    let mock = repository.update(user, name, mock).await?;
    let response = mock.map(Json);

    Ok(response)
}
