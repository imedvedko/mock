use crate::model::BigString;
use crate::model::Result;
use crate::repository::MockData;
use rocket::http::Method;
use rocket::tokio::time::{sleep, Duration};
use rocket::{get, post};
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

#[openapi(tag = "Call")]
#[get("/mocks/<name>/call?<timeout>")]
pub async fn get(
    db: Connection<MockData>,
    name: &str,
    method: Method,
    timeout: Option<u32>,
) -> Result<Option<String>> {
    call(db, name, method, None, timeout).await
}

#[openapi(tag = "Call")]
#[post("/mocks/<name>/call?<timeout>", data = "<request>")]
pub async fn post(
    db: Connection<MockData>,
    name: &str,
    method: Method,
    request: Option<BigString>,
    timeout: Option<u32>,
) -> Result<Option<String>> {
    call(db, name, method, request, timeout).await
}

async fn call(
    mut db: Connection<MockData>,
    name: &str,
    method: Method,
    request: Option<BigString>,
    timeout: Option<u32>,
) -> Result<Option<String>> {
    let response = sqlx::query!("SELECT response FROM mocks WHERE name = ?", name)
        .fetch_optional(&mut **db)
        .await?
        .map(|row| row.response);

    if response.is_some() {
        let method = method.as_str();
        let request = request.map(|request| request.value);

        sqlx::query!(
            "INSERT INTO logs (name, method, request, response, timeout) VALUES (?, ?, ?, ?, ?)",
            name,
            method,
            request,
            response,
            timeout
        )
        .execute(&mut **db)
        .await?;
    }

    if let Some(timeout) = timeout {
        sleep(Duration::from_millis(timeout as u64)).await;
    }

    Ok(response)
}
