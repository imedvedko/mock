use crate::model::BigString;
use crate::model::Mock;
use crate::model::Result;
use crate::model::User;
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
) -> Result<Created<Json<Mock>>> {
    let mock = Mock {
        name,
        response: response.value,
    };
    let mock = repository.create(user, mock).await?;
    let location = rocket::uri!(crate::routing::mock::get(&mock.name)).to_string();

    Ok(Created::new(location).body(Json(mock)))
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
