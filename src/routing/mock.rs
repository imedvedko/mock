use crate::model::Mock;
use crate::model::Result;
use crate::model::User;
use crate::repository::MockData;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;
use rocket_okapi::openapi;
use sqlx::Connection as SqlxConnection;

#[openapi(tag = "Mock")]
#[get("/mocks")]
pub async fn list(mut db: Connection<MockData>, _user: User) -> Result<Json<Vec<String>>> {
    let mocks = sqlx::query!("SELECT name FROM mocks")
        .fetch_all(&mut **db)
        .await?
        .into_iter()
        .map(|row| row.name)
        .collect();

    Ok(Json(mocks))
}

#[openapi(tag = "Mock")]
#[post("/mocks", data = "<mock>")]
pub async fn create(
    mut db: Connection<MockData>,
    user: User,
    mock: Json<Mock>,
) -> Result<Created<Json<Mock>>> {
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

                Ok(mock) as Result<Json<Mock>>
            })
        })
        .await?;
    let location = rocket::uri!(get(&mock.name)).to_string();

    Ok(Created::new(location).body(mock))
}

#[openapi(tag = "Mock")]
#[get("/mocks/<name>")]
pub async fn get(
    mut db: Connection<MockData>,
    _user: User,
    name: String,
) -> Result<Option<Json<Mock>>> {
    let response = sqlx::query!(
        "SELECT response FROM mocks WHERE name = ?",
        name
    )
    .fetch_optional(&mut **db)
    .await?
    .map(|row| {
        Json(Mock {
            name,
            response: row.response,
        })
    });

    Ok(response)
}

#[openapi(tag = "Mock")]
#[put("/mocks/<name>", data = "<mock>")]
pub async fn update(
    db: Connection<MockData>,
    user: User,
    name: String,
    mock: Json<Mock>,
) -> Result<Option<Json<Mock>>> {
    let mock = mock.into_inner();
    let mock = update_mock(db, user, name, mock).await?;
    let response = mock.map(Json);

    Ok(response)
}

async fn update_mock(
    mut db: Connection<MockData>,
    user: User,
    name: String,
    mock: Mock,
) -> Result<Option<Mock>> {
    db.transaction(|transaction| {
        Box::pin(async move {
            let result = sqlx::query!(
                "UPDATE mocks SET name = ?, response = ? WHERE name = ?",
                mock.name,
                mock.response,
                name
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

            Ok(response)
        })
    })
    .await
}

#[openapi(tag = "Mock")]
#[delete("/mocks/<name>")]
pub async fn delete(mut db: Connection<MockData>, user: User, name: String) -> Result<Option<()>> {
    db.transaction(|transaction| {
        Box::pin(async move {
            let result = sqlx::query!("DELETE FROM mocks WHERE name = ?", name)
                .execute(&mut **transaction)
                .await?;

            let response = if result.rows_affected() > 0 {
                sqlx::query!(
                    "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                    user.token,
                    name,
                    None::<String>
                )
                .execute(&mut **transaction)
                .await?;

                Some(())
            } else {
                None
            };

            Ok(response)
        })
    })
    .await
}
