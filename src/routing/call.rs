use crate::model::BigString;
use crate::model::Result;
use crate::repository::MockRepository;
use rocket::http::Method;
use rocket::{get, post};
use rocket_okapi::openapi;

#[openapi(tag = "Call")]
#[get("/mocks/<name>/call?<timeout>")]
pub async fn get(
    mut repository: MockRepository,
    name: String,
    method: Method,
    timeout: Option<u32>,
) -> Result<Option<String>> {
    repository
        .call(name, method.to_string(), None, timeout)
        .await
}

#[openapi(tag = "Call")]
#[post("/mocks/<name>/call?<timeout>", data = "<request>")]
pub async fn post(
    mut repository: MockRepository,
    name: String,
    method: Method,
    request: Option<BigString>,
    timeout: Option<u32>,
) -> Result<Option<String>> {
    repository
        .call(
            name,
            method.to_string(),
            request.map(|request| request.value),
            timeout,
        )
        .await
}
