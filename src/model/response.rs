use super::HttpError;
use rocket::http::{Header, Status};
use rocket::serde::json::Json;
use rocket::Responder;

#[derive(Responder)]
pub enum Response {
    Body {
        inner: Json<HttpError>,
    },
    BodyWithHeader {
        inner: Json<HttpError>,
        header: Header<'static>,
    },
}

impl Response {
    pub fn http_error(status: Status) -> Self {
        let inner = Json(HttpError::new(status, None));
        if status == Status::Unauthorized {
            Self::BodyWithHeader {
                inner,
                header: Header::new("WWW-Authenticate", "Bearer"),
            }
        } else {
            Self::Body { inner }
        }
    }
}
