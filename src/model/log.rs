use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Log {
    pub timestamp: NaiveDateTime,
    pub name: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    pub response: String,
}
