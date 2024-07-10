use super::HttpError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::ensure_status_code_exists;

#[derive(Responder)]
pub enum Conflict<T> {
    Ok { inner: T },
    Error { inner: (Status, Json<HttpError>) },
}

impl<T> Conflict<T> {
    pub fn ok(value: T) -> Self {
        Conflict::Ok { inner: value }
    }

    pub fn error(name: String) -> Self {
        Conflict::Error {
            inner: (
                Status::Conflict,
                Json(HttpError::new(
                    Status::Conflict,
                    Some(format!("{} already exists", name)),
                )),
            ),
        }
    }
}

impl<T: OpenApiResponderInner> OpenApiResponderInner for Conflict<T> {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = T::responses(gen)?;
        ensure_status_code_exists(&mut responses, Status::Conflict.code);
        Ok(responses)
    }
}
