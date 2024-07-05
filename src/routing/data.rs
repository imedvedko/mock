use crate::model::BigString;
use crate::model::Mock;
use crate::model::Result;
use crate::model::User;
use crate::repository::MockData;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{post, put};
use rocket_db_pools::Connection;
use rocket_okapi::openapi;
use sqlx::Connection as SqlxConnection;

#[openapi(tag = "Data")]
#[post("/mocks/<name>/data", data = "<response>")]
pub async fn create(
    mut db: Connection<MockData>,
    user: User,
    name: String,
    response: BigString,
) -> Result<Created<Json<Mock>>> {
    let mock = Mock { name, response: response.value };
    let mock = db
        .transaction(|transaction| {
            Box::pin(async move {
                sqlx::query!(
                    "INSERT INTO mocks (name, response) VALUES (?, ?)",
                    mock.name,
                    mock.response
                )
                .execute(&mut **transaction)
                .await?;

                sqlx::query!(
                    "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                    user.token,
                    mock.name,
                    mock.response
                )
                .execute(&mut **transaction)
                .await?;

                Ok(mock) as Result<Mock>
            })
        })
        .await?;
    let location = rocket::uri!(crate::routing::mock::get(&mock.name)).to_string();

    Ok(Created::new(location).body(Json(mock)))
}

#[openapi(tag = "Data")]
#[put("/mocks/<name>/data", data = "<response>")]
pub async fn update(
    mut db: Connection<MockData>,
    user: User,
    name: String,
    response: BigString,
) -> Result<Option<Json<Mock>>> {
    let mock = Mock { name, response: response.value };
    let mock = db
        .transaction(|transaction| {
            Box::pin(async move {
                let result = sqlx::query!(
                    "UPDATE mocks SET response = ? WHERE name = ?",
                    mock.response,
                    mock.name
                )
                .execute(&mut **transaction)
                .await?;

                let response = if result.rows_affected() > 0 {
                    sqlx::query!(
                        "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                        user.token,
                        mock.name,
                        mock.response
                    )
                    .execute(&mut **transaction)
                    .await?;

                    Some(mock)
                } else {
                    None
                };

                Ok(response) as Result<Option<Mock>>
            })
        })
        .await?;
    let response = mock.map(Json);

    Ok(response)
}
