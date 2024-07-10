use rocket::http::Status;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HttpError {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl HttpError {
    pub fn new(status: Status, description: Option<String>) -> Self {
        Self {
            code: status.code,
            reason: status.reason().map(ToOwned::to_owned),
            description,
        }
    }
}
